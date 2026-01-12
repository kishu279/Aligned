import { Profile, Prompt } from "@/data/profiles";
import { Ionicons } from "@expo/vector-icons";
import React from "react";
import {
    Dimensions,
    Image,
    StyleSheet,
    Text,
    TouchableOpacity,
    View,
} from "react-native";

const { width } = Dimensions.get("window");
const CARD_WIDTH = width - 40;

interface ProfileCardProps {
    profile: Profile;
    onLike: (id: string) => void;
    onPass: (id: string) => void;
}

// Image Card Component
function ImageCard({
    image,
    showLike = false,
    onLike,
}: {
    image: any;
    showLike?: boolean;
    onLike?: () => void;
}) {
    return (
        <View style={styles.imageCard}>
            <Image source={image} style={styles.cardImage} resizeMode="cover" />

            {showLike && (
                <TouchableOpacity style={styles.likeButton} onPress={onLike}>
                    <Ionicons name="heart-outline" size={26} color="#000" />
                </TouchableOpacity>
            )}
        </View>
    );
}

// Prompt Card Component
function PromptCard({
    prompt,
    onLike
}: {
    prompt: Prompt;
    onLike?: () => void;
}) {
    return (
        <View style={styles.promptCard}>
            <Text style={styles.promptQuestion}>{prompt.question}</Text>
            <Text style={styles.promptAnswer}>{prompt.answer}</Text>

            <TouchableOpacity style={styles.promptLikeButton} onPress={onLike}>
                <Ionicons name="heart-outline" size={24} color="#000" />
            </TouchableOpacity>
        </View>
    );
}

// Info Card Component
function InfoCard({ profile }: { profile: Profile }) {
    return (
        <View style={styles.infoCard}>
            {/* Basic Info Row */}
            <View style={styles.infoRow}>
                {profile.age && (
                    <View style={styles.infoChip}>
                        <Ionicons name="person-outline" size={16} color="#000" />
                        <Text style={styles.infoChipText}>{profile.age}</Text>
                    </View>
                )}
                {profile.gender && (
                    <View style={styles.infoChip}>
                        <Ionicons name="male-female-outline" size={16} color="#000" />
                        <Text style={styles.infoChipText}>{profile.gender}</Text>
                    </View>
                )}
                {profile.sexuality && (
                    <View style={styles.infoChip}>
                        <Ionicons name="heart-circle-outline" size={16} color="#000" />
                        <Text style={styles.infoChipText}>{profile.sexuality}</Text>
                    </View>
                )}
                {profile.height && (
                    <View style={styles.infoChip}>
                        <Ionicons name="resize-outline" size={16} color="#000" />
                        <Text style={styles.infoChipText}>{profile.height}</Text>
                    </View>
                )}
            </View>

            {/* Details List */}
            <View style={styles.detailsList}>
                {profile.job && (
                    <View style={styles.detailRow}>
                        <Ionicons name="briefcase-outline" size={20} color="#000" />
                        <Text style={styles.detailText}>{profile.job}</Text>
                    </View>
                )}
                {profile.location && (
                    <View style={styles.detailRow}>
                        <Ionicons name="location-outline" size={20} color="#000" />
                        <Text style={styles.detailText}>{profile.location}</Text>
                    </View>
                )}
                {profile.ethnicity && (
                    <View style={styles.detailRow}>
                        <Ionicons name="globe-outline" size={20} color="#000" />
                        <Text style={styles.detailText}>{profile.ethnicity}</Text>
                    </View>
                )}
                {profile.politics && (
                    <View style={styles.detailRow}>
                        <Ionicons name="flag-outline" size={20} color="#000" />
                        <Text style={styles.detailText}>{profile.politics}</Text>
                    </View>
                )}
                {profile.datingIntention && (
                    <View style={styles.detailRow}>
                        <Ionicons name="search-outline" size={20} color="#000" />
                        <View>
                            <Text style={styles.detailTitle}>Dating Intention</Text>
                            <Text style={styles.detailSubtext}>{profile.datingIntention}</Text>
                        </View>
                    </View>
                )}
                {profile.relationshipType && (
                    <View style={styles.detailRow}>
                        <Ionicons name="people-outline" size={20} color="#000" />
                        <View>
                            <Text style={styles.detailTitle}>{profile.relationshipType}</Text>
                            <Text style={styles.detailSubtext}>I am looking for a true person to connect with.</Text>
                        </View>
                    </View>
                )}
                {profile.drinks && (
                    <View style={styles.detailRow}>
                        <Ionicons name="wine-outline" size={20} color="#000" />
                        <Text style={styles.detailText}>Drinks: {profile.drinks}</Text>
                    </View>
                )}
                {profile.smokes && (
                    <View style={styles.detailRow}>
                        <Ionicons name="leaf-outline" size={20} color="#000" />
                        <Text style={styles.detailText}>Smokes: {profile.smokes}</Text>
                    </View>
                )}
            </View>
        </View>
    );
}

export default function ProfileCard({ profile, onLike, onPass }: ProfileCardProps) {
    const handleLike = () => onLike(profile.id);
    const handlePass = () => onPass(profile.id);

    return (
        <View style={styles.container}>
            {/* First Image with Like button */}
            <ImageCard
                image={profile.images[0]}
                showLike={true}
                onLike={handleLike}
            />

            {/* First Prompt */}
            {profile.prompts[0] && (
                <PromptCard prompt={profile.prompts[0]} onLike={handleLike} />
            )}

            {/* Info Card */}
            <InfoCard profile={profile} />

            {/* Second Image */}
            {profile.images[1] && (
                <ImageCard
                    image={profile.images[1]}
                />
            )}

            {/* Second Prompt */}
            {profile.prompts[1] && (
                <PromptCard prompt={profile.prompts[1]} onLike={handleLike} />
            )}

            {/* Third Image */}
            {profile.images[2] && (
                <ImageCard
                    image={profile.images[2]}
                />
            )}

            {/* Third Prompt */}
            {profile.prompts[2] && (
                <PromptCard prompt={profile.prompts[2]} onLike={handleLike} />
            )}
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        paddingBottom: 40,
    },

    // Image Card Styles
    imageCard: {
        position: "relative",
        marginBottom: 12,
        borderRadius: 12,
        overflow: "hidden",
    },
    cardImage: {
        width: CARD_WIDTH,
        height: CARD_WIDTH * 1.25,
        borderRadius: 12,
    },
    likeButton: {
        position: "absolute",
        bottom: 16,
        right: 16,
        width: 48,
        height: 48,
        borderRadius: 24,
        backgroundColor: "#fff",
        justifyContent: "center",
        alignItems: "center",
        shadowColor: "#000",
        shadowOffset: { width: 0, height: 2 },
        shadowOpacity: 0.15,
        shadowRadius: 4,
        elevation: 4,
    },

    // Prompt Card Styles
    promptCard: {
        backgroundColor: "#fff",
        borderRadius: 12,
        padding: 20,
        paddingBottom: 70,
        marginBottom: 12,
        borderWidth: 1,
        borderColor: "#E5E5E5",
        position: "relative",
        minHeight: 140,
    },
    promptQuestion: {
        fontSize: 14,
        fontWeight: "800",
        color: "#000",
        marginBottom: 8,
        fontFamily: "NunitoSans",
    },
    promptAnswer: {
        fontSize: 22,
        fontWeight: "600",
        color: "#000",
        fontFamily: "NunitoSans",
        lineHeight: 30,
    },
    promptLikeButton: {
        position: "absolute",
        bottom: 16,
        right: 16,
        width: 44,
        height: 44,
        borderRadius: 22,
        backgroundColor: "#fff",
        justifyContent: "center",
        alignItems: "center",
        borderWidth: 1,
        borderColor: "#E5E5E5",
    },

    // Info Card Styles
    infoCard: {
        backgroundColor: "#fff",
        borderRadius: 12,
        padding: 16,
        marginBottom: 12,
        borderWidth: 1,
        borderColor: "#E5E5E5",
    },
    infoRow: {
        flexDirection: "row",
        flexWrap: "wrap",
        gap: 8,
        marginBottom: 16,
        paddingBottom: 16,
        borderBottomWidth: 1,
        borderBottomColor: "#F0F0F0",
    },
    infoChip: {
        flexDirection: "row",
        alignItems: "center",
        paddingHorizontal: 12,
        paddingVertical: 8,
        backgroundColor: "#F5F5F5",
        borderRadius: 20,
        gap: 6,
    },
    infoChipText: {
        fontSize: 14,
        fontWeight: "500",
        color: "#000",
        fontFamily: "NunitoSans",
    },
    detailsList: {
        gap: 16,
    },
    detailRow: {
        flexDirection: "row",
        alignItems: "flex-start",
        gap: 12,
    },
    detailText: {
        fontSize: 15,
        fontWeight: "500",
        color: "#000",
        fontFamily: "NunitoSans",
    },
    detailTitle: {
        fontSize: 15,
        fontWeight: "600",
        color: "#000",
        fontFamily: "NunitoSans",
    },
    detailSubtext: {
        fontSize: 13,
        fontWeight: "500",
        color: "#333",
        marginTop: 2,
        fontFamily: "NunitoSans",
    },
});
