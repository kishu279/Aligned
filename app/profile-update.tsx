import { Ionicons } from "@expo/vector-icons";
import { useRouter } from "expo-router";
import React, { useEffect, useState } from "react";
import {
  ScrollView,
  StyleSheet,
  Text,
  TouchableOpacity,
  View,
  ActivityIndicator,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { getMyProfile, ProfileDetails } from "../lib/api/endpoints";

export default function ProfileUpdate() {
  const router = useRouter();
  const [profile, setProfile] = useState<ProfileDetails | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchProfile();
  }, []);

  const fetchProfile = async () => {
    try {
      setLoading(true);
      const profileResult = await getMyProfile();
      setProfile(profileResult.details || null);
    } catch (error) {
      console.error("Failed to fetch profile:", error);
    } finally {
      setLoading(false);
    }
  };

  const PreferenceItem = ({
    title,
    value,
    isMissing,
  }: {
    title: string;
    value: string;
    isMissing?: boolean;
  }) => (
    <View
      style={[
        styles.itemContainer,
        isMissing && styles.missingFieldContainer,
      ]}
    >
      <View style={styles.itemContent}>
        <View style={styles.itemHeader}>
          <Text style={styles.itemTitle}>{title}</Text>
          {isMissing && (
            <View style={styles.missingBadge}>
              <Ionicons name="alert-circle" size={14} color="#D24E4E" />
              <Text style={styles.missingText}>Missing</Text>
            </View>
          )}
        </View>
        <Text
          style={[styles.itemValue, isMissing && styles.missingValue]}
        >
          {value}
        </Text>
      </View>
      <Ionicons name="lock-closed-outline" size={20} color="#D24E4E" />
    </View>
  );

  return (
    <View style={styles.container}>
      <SafeAreaView style={styles.safeArea} edges={["top"]}>
        <View style={styles.header}>
          <TouchableOpacity
            onPress={() => router.back()}
            style={styles.backButton}
          >
            <Ionicons name="chevron-back" size={28} color="#000" />
          </TouchableOpacity>
          <Text style={styles.headerTitle}>Profile Info</Text>
          <View style={{ width: 28 }} />
        </View>

        <ScrollView contentContainerStyle={styles.scrollContent}>
          {loading ? (
            <View style={styles.loadingContainer}>
              <ActivityIndicator size="large" color="#7D4ca2" />
            </View>
          ) : (
            <>
              <View style={styles.upgradeBanner}>
                <TouchableOpacity style={styles.upgradeButton}>
                  <Text style={styles.upgradeButtonText}>Edit Profile</Text>
                </TouchableOpacity>
                <Text style={styles.upgradeText}>
                  Update these on your profile page.
                </Text>
              </View>

              <PreferenceItem
                title="Height"
                value={profile?.height ? `${profile.height} cm` : "Not set"}
                isMissing={!profile?.height}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Dating Intentions"
                value={profile?.dating_intention || "Not set"}
                isMissing={!profile?.dating_intention}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Relationship Type"
                value={profile?.relationship_type || "Not set"}
                isMissing={!profile?.relationship_type}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Smoking"
                value={profile?.smokes || "Not set"}
                isMissing={!profile?.smokes}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Drinking"
                value={profile?.drinks || "Not set"}
                isMissing={!profile?.drinks}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Politics"
                value={profile?.politics || "Not set"}
                isMissing={!profile?.politics}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Education"
                value={profile?.school || "Not set"}
                isMissing={!profile?.school}
              />
              <View style={styles.divider} />

              <View style={{ height: 100 }} />
            </>
          )}
        </ScrollView>
      </SafeAreaView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#fff",
  },
  safeArea: {
    flex: 1,
  },
  header: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: 16,
    paddingBottom: 12,
    borderBottomWidth: 1,
    borderBottomColor: "#f0f0f0",
  },
  backButton: {
    padding: 4,
  },
  headerTitle: {
    fontSize: 18,
    fontWeight: "900",
    color: "#000",
    fontFamily: "NunitoSans",
  },
  scrollContent: {
    paddingBottom: 40,
    paddingTop: 20,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    paddingVertical: 60,
  },
  itemContainer: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    paddingVertical: 16,
    paddingHorizontal: 20,
  },
  itemContent: {
    flex: 1,
    paddingRight: 16,
  },
  itemHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginBottom: 4,
  },
  itemTitle: {
    fontSize: 16,
    fontWeight: "900",
    color: "#000",
    fontFamily: "NunitoSans",
  },
  itemValue: {
    fontSize: 14,
    color: "#000",
    fontFamily: "NunitoSans",
  },
  divider: {
    height: 1,
    backgroundColor: "#f0f0f0",
    marginLeft: 20,
  },
  missingFieldContainer: {
    backgroundColor: "#FFF5F5",
    borderLeftWidth: 3,
    borderLeftColor: "#D24E4E",
  },
  missingBadge: {
    flexDirection: "row",
    alignItems: "center",
    gap: 4,
    backgroundColor: "#FFE5E5",
    paddingHorizontal: 8,
    paddingVertical: 3,
    borderRadius: 12,
  },
  missingText: {
    fontSize: 11,
    color: "#D24E4E",
    fontFamily: "NunitoSans",
    fontWeight: "700",
  },
  missingValue: {
    color: "#D24E4E",
    fontStyle: "italic",
  },
  upgradeBanner: {
    backgroundColor: "#F3EBF5",
    marginHorizontal: 20,
    marginVertical: 16,
    padding: 16,
    borderRadius: 12,
    flexDirection: "row",
    alignItems: "center",
    gap: 16,
  },
  upgradeButton: {
    backgroundColor: "#fff",
    paddingVertical: 10,
    paddingHorizontal: 20,
    borderRadius: 20,
    shadowColor: "#000",
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.1,
    shadowRadius: 2,
    elevation: 1,
  },
  upgradeButtonText: {
    fontSize: 14,
    fontWeight: "900",
    color: "#7D4ca2",
  },
  upgradeText: {
    flex: 1,
    fontSize: 14,
    color: "#000",
    fontFamily: "NunitoSans",
    fontWeight: "600",
    lineHeight: 20,
  },
});
