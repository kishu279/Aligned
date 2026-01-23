import { getUploadUrl, getMyProfile } from "@/lib/api/endpoints";
import { Ionicons } from "@expo/vector-icons";
import * as ImagePicker from "expo-image-picker";
import { useRouter } from "expo-router";
import React, { useEffect, useState } from "react";
import {
    ActivityIndicator,
    Alert,
    Image,
    StyleSheet,
    Text,
    TouchableOpacity,
    View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

interface ImageSlot {
    uri: string | null;
    isUploading: boolean;
    uploadedKey: string | null;
}

const TOTAL_SLOTS = 6;

export default function ImagesScreen() {
    const router = useRouter();
    const [imageSlots, setImageSlots] = useState<ImageSlot[]>(
        Array(TOTAL_SLOTS).fill({ uri: null, isUploading: false, uploadedKey: null })
    );
    const [isLoading, setIsLoading] = useState(false);
    const [isSaving, setIsSaving] = useState(false);

    const filledSlots = imageSlots.filter((slot) => slot.uri !== null).length;
    const canContinue = filledSlots >= 1; // Require at least 1 image

    useEffect(() => {
        loadExistingImages();
    }, []);

    const loadExistingImages = async () => {
        setIsLoading(true);
        try {
            // Use getMyProfile which now returns presigned download URLs
            const response = await getMyProfile();
            if (response.images && response.images.length > 0) {
                const newSlots = [...imageSlots];
                response.images.forEach((image, index) => {
                    if (index < TOTAL_SLOTS) {
                        newSlots[index] = { uri: image.url, isUploading: false, uploadedKey: image.id };
                    }
                });
                setImageSlots(newSlots);
            }
        } catch (error) {
            console.log("[Images] No existing images found:", error);
        } finally {
            setIsLoading(false);
        }
    };

    const pickImage = async (slotIndex: number) => {
        // Request permission
        const permissionResult = await ImagePicker.requestMediaLibraryPermissionsAsync();
        if (!permissionResult.granted) {
            Alert.alert("Permission Required", "Please allow access to your photo library.");
            return;
        }

        // Pick image
        const result = await ImagePicker.launchImageLibraryAsync({
            mediaTypes: ImagePicker.MediaTypeOptions.Images,
            allowsEditing: true,
            aspect: [3, 4],
            quality: 0.8,
        });

        if (!result.canceled && result.assets[0]) {
            const imageUri = result.assets[0].uri;
            await uploadImage(slotIndex, imageUri);
        }
    };

    const uploadImage = async (slotIndex: number, uri: string) => {
        // Update slot to show loading
        const newSlots = [...imageSlots];
        newSlots[slotIndex] = { uri, isUploading: true, uploadedKey: null };
        setImageSlots(newSlots);

        try {
            // Get filename and content type from URI
            const filename = uri.split("/").pop() || `image_${slotIndex}.jpg`;
            const contentType = filename.endsWith(".png") ? "image/png" : "image/jpeg";

            // Get presigned upload URL
            const { upload_url, key } = await getUploadUrl(filename, contentType);

            // Upload the file to R2
            const response = await fetch(uri);
            const blob = await response.blob();

            await fetch(upload_url, {
                method: "PUT",
                headers: {
                    "Content-Type": contentType,
                },
                body: blob,
            });

            // Update slot with success
            newSlots[slotIndex] = { uri, isUploading: false, uploadedKey: key };
            setImageSlots([...newSlots]);

            console.log(`[Images] Successfully uploaded image ${slotIndex + 1}`);
        } catch (error: any) {
            console.error("[Images] Upload failed:", error);
            // Reset slot on failure
            newSlots[slotIndex] = { uri: null, isUploading: false, uploadedKey: null };
            setImageSlots([...newSlots]);
            Alert.alert("Upload Failed", error.message || "Failed to upload image");
        }
    };

    const removeImage = (slotIndex: number) => {
        Alert.alert("Remove Image", "Are you sure you want to remove this image?", [
            { text: "Cancel", style: "cancel" },
            {
                text: "Remove",
                style: "destructive",
                onPress: () => {
                    const newSlots = [...imageSlots];
                    newSlots[slotIndex] = { uri: null, isUploading: false, uploadedKey: null };
                    setImageSlots(newSlots);
                },
            },
        ]);
    };

    const handleContinue = async () => {
        if (!canContinue) {
            Alert.alert("Add Photos", "Please add at least 1 photo to continue.");
            return;
        }

        setIsSaving(true);
        try {
            // All images are already uploaded via presigned URLs
            // Just navigate to next screen
            Alert.alert("Success", "Your photos have been saved!", [
                { text: "OK", onPress: () => router.back() },
            ]);
        } catch (error: any) {
            Alert.alert("Error", error.message || "Failed to save images");
        } finally {
            setIsSaving(false);
        }
    };

    const renderImageSlot = (index: number) => {
        const slot = imageSlots[index];
        const isPrimary = index === 0;

        return (
            <TouchableOpacity
                key={index}
                style={[
                    styles.imageSlot,
                    isPrimary && styles.primarySlot,
                    slot.uri && styles.filledSlot,
                ]}
                onPress={() => (slot.uri ? removeImage(index) : pickImage(index))}
                disabled={slot.isUploading}
            >
                {slot.isUploading ? (
                    <View style={styles.loadingContainer}>
                        <ActivityIndicator size="large" color="#8B5A9C" />
                        <Text style={styles.loadingText}>Uploading...</Text>
                    </View>
                ) : slot.uri ? (
                    <>
                        <Image source={{ uri: slot.uri }} style={styles.image} />
                        <View style={styles.removeOverlay}>
                            <View style={styles.removeButton}>
                                <Ionicons name="close" size={16} color="#fff" />
                            </View>
                        </View>
                        {isPrimary && (
                            <View style={styles.primaryBadge}>
                                <Text style={styles.primaryBadgeText}>Main</Text>
                            </View>
                        )}
                    </>
                ) : (
                    <View style={styles.emptySlot}>
                        <Ionicons name="add" size={32} color="#8B5A9C" />
                        {isPrimary && <Text style={styles.addMainText}>Add Main Photo</Text>}
                    </View>
                )}
            </TouchableOpacity>
        );
    };

    if (isLoading) {
        return (
            <SafeAreaView style={styles.container}>
                <View style={styles.loadingScreen}>
                    <ActivityIndicator size="large" color="#8B5A9C" />
                    <Text style={styles.loadingScreenText}>Loading your photos...</Text>
                </View>
            </SafeAreaView>
        );
    }

    return (
        <SafeAreaView style={styles.container}>
            {/* Header */}
            <View style={styles.header}>
                <TouchableOpacity onPress={() => router.back()} style={styles.backButton}>
                    <Ionicons name="chevron-back" size={28} color="#000" />
                </TouchableOpacity>
                <Text style={styles.headerTitle}>Add Photos</Text>
                <View style={styles.headerRight} />
            </View>

            {/* Subtitle */}
            <View style={styles.subtitleContainer}>
                <Text style={styles.subtitle}>
                    Add up to 6 photos to your profile. Your first photo will be your main profile picture.
                </Text>
                <Text style={styles.counter}>
                    {filledSlots}/{TOTAL_SLOTS} photos added
                </Text>
            </View>

            {/* Image Grid */}
            <View style={styles.imageGrid}>
                {/* First row - Primary large + 2 small */}
                <View style={styles.firstRow}>
                    {renderImageSlot(0)}
                    <View style={styles.smallColumn}>
                        {renderImageSlot(1)}
                        {renderImageSlot(2)}
                    </View>
                </View>

                {/* Second row - 3 equal slots */}
                <View style={styles.secondRow}>
                    {renderImageSlot(3)}
                    {renderImageSlot(4)}
                    {renderImageSlot(5)}
                </View>
            </View>

            {/* Tips */}
            <View style={styles.tipsContainer}>
                <Text style={styles.tipsTitle}>Photo Tips</Text>
                <View style={styles.tipRow}>
                    <Ionicons name="checkmark-circle" size={16} color="#4CAF50" />
                    <Text style={styles.tipText}>Clear, well-lit photos of your face</Text>
                </View>
                <View style={styles.tipRow}>
                    <Ionicons name="checkmark-circle" size={16} color="#4CAF50" />
                    <Text style={styles.tipText}>Show your personality and hobbies</Text>
                </View>
                <View style={styles.tipRow}>
                    <Ionicons name="close-circle" size={16} color="#f44336" />
                    <Text style={styles.tipText}>Avoid group photos as your main picture</Text>
                </View>
            </View>

            {/* Continue Button */}
            <View style={styles.buttonContainer}>
                <TouchableOpacity
                    style={[styles.continueButton, !canContinue && styles.continueButtonDisabled]}
                    onPress={handleContinue}
                    disabled={!canContinue || isSaving}
                >
                    {isSaving ? (
                        <ActivityIndicator color="#fff" />
                    ) : (
                        <Text style={styles.continueButtonText}>
                            {canContinue ? "Save Photos" : `Add at least 1 photo`}
                        </Text>
                    )}
                </TouchableOpacity>
            </View>
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#fff",
    },
    loadingScreen: {
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
    },
    loadingScreenText: {
        marginTop: 16,
        fontSize: 16,
        color: "#666",
        fontFamily: "NunitoSans",
    },
    header: {
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "space-between",
        paddingHorizontal: 16,
        paddingVertical: 12,
        borderBottomWidth: 1,
        borderBottomColor: "#f0f0f0",
    },
    backButton: {
        padding: 4,
    },
    headerTitle: {
        fontSize: 18,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#000",
    },
    headerRight: {
        width: 36,
    },
    subtitleContainer: {
        paddingHorizontal: 20,
        paddingVertical: 16,
    },
    subtitle: {
        fontSize: 15,
        color: "#666",
        fontFamily: "NunitoSans",
        lineHeight: 22,
    },
    counter: {
        fontSize: 14,
        color: "#8B5A9C",
        fontFamily: "NunitoSans",
        fontWeight: "600",
        marginTop: 8,
    },
    imageGrid: {
        paddingHorizontal: 20,
    },
    firstRow: {
        flexDirection: "row",
        gap: 8,
        marginBottom: 8,
    },
    primarySlot: {
        flex: 2,
        aspectRatio: 3 / 4,
    },
    smallColumn: {
        flex: 1,
        gap: 8,
    },
    secondRow: {
        flexDirection: "row",
        gap: 8,
    },
    imageSlot: {
        flex: 1,
        aspectRatio: 3 / 4,
        backgroundColor: "#f8f4fa",
        borderRadius: 16,
        overflow: "hidden",
        borderWidth: 2,
        borderColor: "#e0d5e5",
        borderStyle: "dashed",
    },
    filledSlot: {
        borderStyle: "solid",
        borderColor: "#8B5A9C",
    },
    emptySlot: {
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
    },
    addMainText: {
        marginTop: 8,
        fontSize: 12,
        color: "#8B5A9C",
        fontFamily: "NunitoSans",
        fontWeight: "600",
    },
    image: {
        width: "100%",
        height: "100%",
        resizeMode: "cover",
    },
    loadingContainer: {
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
    },
    loadingText: {
        marginTop: 8,
        fontSize: 12,
        color: "#8B5A9C",
        fontFamily: "NunitoSans",
    },
    removeOverlay: {
        position: "absolute",
        top: 8,
        right: 8,
    },
    removeButton: {
        backgroundColor: "rgba(0,0,0,0.6)",
        borderRadius: 12,
        padding: 4,
    },
    primaryBadge: {
        position: "absolute",
        bottom: 8,
        left: 8,
        backgroundColor: "#8B5A9C",
        paddingHorizontal: 10,
        paddingVertical: 4,
        borderRadius: 12,
    },
    primaryBadgeText: {
        fontSize: 12,
        color: "#fff",
        fontFamily: "NunitoSans",
        fontWeight: "600",
    },
    tipsContainer: {
        marginTop: 24,
        paddingHorizontal: 20,
    },
    tipsTitle: {
        fontSize: 16,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#000",
        marginBottom: 12,
    },
    tipRow: {
        flexDirection: "row",
        alignItems: "center",
        gap: 8,
        marginBottom: 8,
    },
    tipText: {
        fontSize: 14,
        color: "#666",
        fontFamily: "NunitoSans",
    },
    buttonContainer: {
        flex: 1,
        justifyContent: "flex-end",
        padding: 20,
    },
    continueButton: {
        backgroundColor: "#8B5A9C",
        paddingVertical: 16,
        borderRadius: 30,
        alignItems: "center",
    },
    continueButtonDisabled: {
        backgroundColor: "#ddd",
    },
    continueButtonText: {
        fontSize: 16,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#fff",
    },
});
