import { getMyProfile } from "@/lib/api/endpoints";
import { useRouter } from "expo-router";
import { useEffect, useState } from "react";
import {
    ActivityIndicator,
    StyleSheet,
    Text,
    View
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

export default function InterstitialScreen() {
    const router = useRouter();
    const [isLoading, setIsLoading] = useState(true);
    const [message, setMessage] = useState("Checking your profile...");

    const handleGetProfile = async () => {
        try {
            setMessage("Loading your profile...");
            const profile = await getMyProfile();

            // Check if profile has required details
            if (profile.details?.name) {
                // Profile exists, go to main tabs
                console.log("[Interstitial] Profile found, going to tabs");
                router.replace("/(tabs)");
            } else {
                // Profile incomplete, go to setup
                console.log("[Interstitial] Profile incomplete, going to setup");
                setMessage("Let's set up your profile!");
                setTimeout(() => router.replace("/auth/profile"), 1000);
            }
        } catch (error: any) {
            console.log("[Interstitial] Profile not found or error:", error);
            // No profile found - new user, go to profile setup
            setMessage("Welcome! Let's create your profile.");
            setTimeout(() => router.replace("/auth/profile"), 1000);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        handleGetProfile();
    }, []);

    return (
        <SafeAreaView style={styles.container}>
            <View style={styles.content}>
                <View style={styles.textContainer}>
                    <Text style={styles.title}>You're one of a kind.</Text>
                    <Text style={styles.title}>Your profile should</Text>
                    <Text style={styles.title}>be, too.</Text>
                </View>

                {/* Loading indicator */}
                <View style={styles.loadingContainer}>
                    {isLoading ? (
                        <ActivityIndicator size="large" color="#8B5A9C" />
                    ) : null}
                    <Text style={styles.message}>{message}</Text>
                </View>

                {/* Illustration */}
                <View style={styles.illustration}>
                    <View style={styles.blob}>
                        <View style={styles.eyesRow}>
                            <View style={styles.eye}>
                                <View style={styles.pupil} />
                            </View>
                            <View style={styles.eye}>
                                <View style={styles.pupil} />
                            </View>
                        </View>
                        <View style={styles.line} />
                    </View>
                </View>
            </View>
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#fff",
    },
    content: {
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
        paddingHorizontal: 32,
    },
    textContainer: {
        marginBottom: 40,
    },
    title: {
        fontSize: 32,
        fontWeight: "900",
        fontFamily: "Tinos-Bold",
        color: "#000",
        textAlign: "left",
        lineHeight: 40,
    },
    loadingContainer: {
        alignItems: "center",
        marginBottom: 40,
    },
    message: {
        marginTop: 16,
        fontSize: 16,
        color: "#666",
        fontFamily: "NunitoSans",
    },
    illustration: {
        alignItems: 'center',
        justifyContent: 'center',
    },
    blob: {
        width: 200,
        height: 180,
        backgroundColor: '#F5F0F7',
        borderRadius: 100,
        justifyContent: 'flex-end',
        paddingBottom: 20,
        alignItems: 'center',
    },
    eyesRow: {
        flexDirection: 'row',
        gap: 4,
        marginBottom: 2,
    },
    eye: {
        width: 32,
        height: 32,
        borderRadius: 16,
        backgroundColor: '#fff',
        borderWidth: 2,
        borderColor: '#000',
        justifyContent: 'center',
        alignItems: 'center',
    },
    pupil: {
        width: 12,
        height: 12,
        borderRadius: 6,
        backgroundColor: '#000',
    },
    line: {
        width: 120,
        height: 2,
        backgroundColor: '#000',
        marginTop: 0,
    }
});
