import ProfileCard from "@/components/ProfileCard";
import { getFeed, interact, UserProfile } from "@/lib/api/endpoints";
import { useAuth } from "@/lib/context/AuthContext";
import { Ionicons } from "@expo/vector-icons";
import { useRouter } from "expo-router";
import React, { useCallback, useEffect, useRef, useState } from "react";
import {
  ActivityIndicator,
  RefreshControl,
  ScrollView,
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

export default function Index() {
  const router = useRouter();
  const { isAuthenticated, isLoading: authLoading } = useAuth();
  const [profiles, setProfiles] = useState<UserProfile[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const scrollViewRef = useRef<ScrollView>(null);
  const [user, setUser] = React.useState<null>(null);

  const currentProfile = profiles[currentIndex];

  // Fetch feed from API
  const fetchFeed = useCallback(async () => {
    try {
      setError(null);
      const response = await getFeed({
        email: "souravpoddar6677@gmail.com"
      });
      setProfiles(response.profiles);
      setCurrentIndex(0);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load feed");
    } finally {
      setIsLoading(false);
      setIsRefreshing(false);
    }
  }, []);

  useEffect(() => {
    if (isAuthenticated) {
      fetchFeed();
    }
  }, [isAuthenticated, fetchFeed]);

  const onRefresh = useCallback(() => {
    setIsRefreshing(true);
    fetchFeed();
  }, [fetchFeed]);

  const goToNextProfile = () => {
    if (profiles.length === 0) return;
    const nextIndex = (currentIndex + 1) % profiles.length;
    setCurrentIndex(nextIndex);
    scrollViewRef.current?.scrollTo({ y: 0, animated: false });
  };

  const handleLike = async (id: string) => {
    try {
      await interact({
        targetUserId: id,
        action: "LIKE",
      });
      console.log("Liked profile:", id);
    } catch (err) {
      console.error("Failed to like:", err);
    }
    goToNextProfile();
  };

  const handlePass = async (id: string) => {
    try {
      await interact({
        targetUserId: id,
        action: "PASS",
      });
      console.log("Passed profile:", id);
    } catch (err) {
      console.error("Failed to pass:", err);
    }
    goToNextProfile();
  };

  // Show loading while auth is initializing
  if (authLoading) {
    return (
      <View style={styles.centerContainer}>
        <ActivityIndicator size="large" color="#8B5A9C" />
      </View>
    );
  }

  // Redirect to auth if not authenticated
  if (!isAuthenticated) {
    return (
      <View style={styles.centerContainer}>
        <Text style={styles.emptyTitle}>Welcome to Aligned</Text>
        <Text style={styles.emptySubtitle}>Sign in to start discovering matches</Text>
        <TouchableOpacity
          style={styles.signInButton}
          onPress={() => router.push("/auth/phone")}
        >
          <Text style={styles.signInButtonText}>Sign In</Text>
        </TouchableOpacity>
      </View>
    );
  }

  // Loading feed
  if (isLoading) {
    return (
      <View style={styles.centerContainer}>
        <ActivityIndicator size="large" color="#8B5A9C" />
        <Text style={styles.loadingText}>Loading profiles...</Text>
      </View>
    );
  }

  // Error state
  if (error) {
    return (
      <View style={styles.centerContainer}>
        <Ionicons name="alert-circle-outline" size={48} color="#999" />
        <Text style={styles.emptyTitle}>Something went wrong</Text>
        <Text style={styles.emptySubtitle}>{error}</Text>
        <TouchableOpacity style={styles.resetButton} onPress={fetchFeed}>
          <Text style={styles.resetButtonText}>Try Again</Text>
        </TouchableOpacity>
      </View>
    );
  }

  // Empty feed
  if (profiles.length === 0) {
    return (
      <View style={styles.centerContainer}>
        <Ionicons name="people-outline" size={48} color="#999" />
        <Text style={styles.emptyTitle}>No profiles yet</Text>
        <Text style={styles.emptySubtitle}>
          Check back later or update your preferences
        </Text>
        <TouchableOpacity style={styles.resetButton} onPress={fetchFeed}>
          <Text style={styles.resetButtonText}>Refresh</Text>
        </TouchableOpacity>
      </View>
    );
  }

  // Transform API profile to component format
  const profileForCard = currentProfile ? {
    id: currentProfile.id,
    name: currentProfile.details?.name || "Unknown",
    age: 25, // TODO: calculate from birthdate
    location: currentProfile.details?.location || "",
    job: currentProfile.details?.job || "",
    height: currentProfile.details?.height ? `${Math.floor(currentProfile.details.height / 30.48)}'${Math.round((currentProfile.details.height % 30.48) / 2.54)}"` : "",
    bio: currentProfile.details?.bio || "",
    religion: currentProfile.details?.religion || "",
    politics: currentProfile.details?.politics || "",
    relationshipType: currentProfile.details?.relationship_type || "",
    datingIntention: currentProfile.details?.dating_intention || "",
    images: currentProfile.images?.map(img => img.url) || [
      "https://via.placeholder.com/400x600?text=No+Image"
    ],
    prompts: currentProfile.prompts?.map(p => ({
      question: p.question,
      answer: p.answer,
    })) || [],
  } : null;

  return (
    <View className="flex-1 bg-white">
      <SafeAreaView className="flex-1" edges={["top"]}>
        {/* Header */}
        <View style={styles.header}>
          <Text style={styles.headerTitle}>{currentProfile?.details?.name || "Aligned"}</Text>
          <TouchableOpacity style={styles.headerButton}>
            <Ionicons name="ellipsis-horizontal" size={24} color="#000" />
          </TouchableOpacity>
        </View>

        {/* Profile Prompt Banner */}
        <View style={styles.promptBanner}>
          <TouchableOpacity
            style={styles.editProfileButton}
            onPress={() => router.push("/(tabs)/profile")}
          >
            <Text style={styles.editProfileText}>Edit profile</Text>
          </TouchableOpacity>
          <Text style={styles.promptText}>
            Complete your profile to send and receive messages, likes, and roses.
          </Text>
        </View>

        {/* Profile Content */}
        <ScrollView
          ref={scrollViewRef}
          style={styles.scrollView}
          contentContainerStyle={styles.scrollContent}
          showsVerticalScrollIndicator={false}
          refreshControl={
            <RefreshControl refreshing={isRefreshing} onRefresh={onRefresh} />
          }
        >
          {profileForCard && (
            <ProfileCard
              profile={profileForCard}
              onLike={handleLike}
              onPass={handlePass}
            />
          )}

          {/* Bottom padding for tab bar */}
          <View style={{ height: 120 }} />
        </ScrollView>

        {/* Static X button - fixed at bottom */}
        <View style={styles.staticPassButtonContainer}>
          <TouchableOpacity
            style={styles.staticPassButton}
            onPress={() => currentProfile && handlePass(currentProfile.id)}
          >
            <Ionicons name="close" size={28} color="#000" />
          </TouchableOpacity>
        </View>
      </SafeAreaView>
    </View>
  );
}

const styles = StyleSheet.create({
  centerContainer: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    backgroundColor: "#fff",
    paddingHorizontal: 40,
  },
  loadingText: {
    marginTop: 16,
    fontSize: 16,
    color: "#666",
    fontFamily: "NunitoSans",
  },
  header: {
    flexDirection: "row",
    justifyContent: "center",
    alignItems: "center",
    paddingHorizontal: 20,
    paddingVertical: 12,
    position: "relative",
  },
  headerTitle: {
    fontSize: 18,
    fontWeight: "600",
    color: "#000",
    fontFamily: "NunitoSans",
  },
  headerButton: {
    position: "absolute",
    right: 20,
    padding: 4,
  },
  promptBanner: {
    flexDirection: "row",
    alignItems: "center",
    marginHorizontal: 20,
    padding: 14,
    backgroundColor: "#FBE8E7",
    borderRadius: 12,
    gap: 12,
  },
  editProfileButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    backgroundColor: "#fff",
    borderRadius: 20,
  },
  editProfileText: {
    fontSize: 14,
    fontWeight: "600",
    color: "#000",
    fontFamily: "NunitoSans",
  },
  promptText: {
    flex: 1,
    fontSize: 13,
    color: "#333",
    fontFamily: "NunitoSans",
  },
  scrollView: {
    flex: 1,
  },
  scrollContent: {
    padding: 20,
    paddingTop: 16,
  },
  emptyTitle: {
    fontSize: 22,
    fontWeight: "600",
    color: "#000",
    marginTop: 20,
    fontFamily: "NunitoSans",
  },
  emptySubtitle: {
    fontSize: 15,
    color: "#666",
    marginTop: 8,
    textAlign: "center",
    fontFamily: "NunitoSans",
  },
  resetButton: {
    marginTop: 24,
    paddingHorizontal: 24,
    paddingVertical: 12,
    backgroundColor: "#000",
    borderRadius: 25,
  },
  resetButtonText: {
    fontSize: 15,
    fontWeight: "600",
    color: "#fff",
    fontFamily: "NunitoSans",
  },
  signInButton: {
    marginTop: 24,
    paddingHorizontal: 32,
    paddingVertical: 14,
    backgroundColor: "#8B5A9C",
    borderRadius: 25,
  },
  signInButtonText: {
    fontSize: 16,
    fontWeight: "600",
    color: "#fff",
    fontFamily: "NunitoSans",
  },
  staticPassButtonContainer: {
    position: "absolute",
    bottom: 100,
    left: 20,
    zIndex: 100,
  },
  staticPassButton: {
    width: 56,
    height: 56,
    borderRadius: 28,
    backgroundColor: "#fff",
    justifyContent: "center",
    alignItems: "center",
    shadowColor: "#000",
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.2,
    shadowRadius: 8,
    elevation: 6,
    borderWidth: 1,
    borderColor: "#E5E5E5",
  },
});
