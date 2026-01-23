import { createPrompt, getPrompts, UserPrompt } from "@/lib/api/endpoints";
import { Ionicons } from "@expo/vector-icons";
import { useRouter } from "expo-router";
import React, { useState } from "react";
import {
    ActivityIndicator,
    Alert,
    FlatList,
    KeyboardAvoidingView,
    Modal,
    Platform,
    StyleSheet,
    Text,
    TextInput,
    TouchableOpacity,
    View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

// Sample prompt questions that users can choose from
const PROMPT_QUESTIONS = [
    "The way to win me over is",
    "I'm looking for",
    "My simple pleasures",
    "Unusual skills",
    "Dating me is like",
    "I'm convinced that",
    "A life goal of mine",
    "My love language is",
    "The key to my heart is",
    "I get along best with people who",
    "Something I learned recently",
    "My most irrational fear",
    "Two truths and a lie",
    "I won't shut up about",
    "The one thing I'd love to know about you is",
];

interface PromptAnswer {
    question: string;
    answer: string;
}

export default function PromptsScreen() {
    const router = useRouter();
    const [selectedPrompts, setSelectedPrompts] = useState<PromptAnswer[]>([]);
    const [currentQuestion, setCurrentQuestion] = useState<string | null>(null);
    const [answerText, setAnswerText] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const [showQuestionPicker, setShowQuestionPicker] = useState(false);

    const maxPrompts = 3;
    const canAddMore = selectedPrompts.length < maxPrompts;

    // Get available questions (not already selected)
    const availableQuestions = PROMPT_QUESTIONS.filter(
        (q) => !selectedPrompts.some((p) => p.question === q)
    );

    const handleSelectQuestion = (question: string) => {
        setCurrentQuestion(question);
        setShowQuestionPicker(false);
        setAnswerText("");
    };

    const handleSaveAnswer = () => {
        if (!currentQuestion || !answerText.trim()) {
            Alert.alert("Error", "Please enter an answer");
            return;
        }

        if (answerText.trim().length < 10) {
            Alert.alert("Error", "Answer should be at least 10 characters");
            return;
        }

        setSelectedPrompts([
            ...selectedPrompts,
            { question: currentQuestion, answer: answerText.trim() },
        ]);
        setCurrentQuestion(null);
        setAnswerText("");
    };

    const handleRemovePrompt = (index: number) => {
        setSelectedPrompts(selectedPrompts.filter((_, i) => i !== index));
    };

    const handleEditPrompt = (index: number) => {
        const prompt = selectedPrompts[index];
        setCurrentQuestion(prompt.question);
        setAnswerText(prompt.answer);
        setSelectedPrompts(selectedPrompts.filter((_, i) => i !== index));
    };

    const handleSubmitAll = async () => {
        if (selectedPrompts.length === 0) {
            Alert.alert("Error", "Please add at least one prompt");
            return;
        }

        setIsLoading(true);
        try {
            // Submit all prompts to the backend
            for (const prompt of selectedPrompts) {
                await createPrompt(prompt.question, prompt.answer);
            }

            Alert.alert("Success", "Your prompts have been saved!", [
                { text: "OK", onPress: () => router.back() },
            ]);
        } catch (error: any) {
            Alert.alert("Error", error.message || "Failed to save prompts");
        } finally {
            setIsLoading(false);
        }
    };

    const handleCancel = () => {
        setCurrentQuestion(null);
        setAnswerText("");
    };

    // Render a saved prompt card
    const renderPromptCard = ({ item, index }: { item: PromptAnswer; index: number }) => (
        <View style={styles.promptCard}>
            <View style={styles.promptHeader}>
                <Text style={styles.promptQuestion}>{item.question}</Text>
                <View style={styles.promptActions}>
                    <TouchableOpacity onPress={() => handleEditPrompt(index)} style={styles.actionButton}>
                        <Ionicons name="pencil" size={18} color="#8B5A9C" />
                    </TouchableOpacity>
                    <TouchableOpacity onPress={() => handleRemovePrompt(index)} style={styles.actionButton}>
                        <Ionicons name="trash-outline" size={18} color="#f44336" />
                    </TouchableOpacity>
                </View>
            </View>
            <Text style={styles.promptAnswer}>{item.answer}</Text>
        </View>
    );

    return (
        <SafeAreaView style={styles.container}>
            <KeyboardAvoidingView
                behavior={Platform.OS === "ios" ? "padding" : "height"}
                style={styles.keyboardAvoidingView}
            >
                {/* Header */}
                <View style={styles.header}>
                    <TouchableOpacity onPress={() => router.back()} style={styles.backButton}>
                        <Ionicons name="chevron-back" size={28} color="#000" />
                    </TouchableOpacity>
                    <Text style={styles.headerTitle}>Your Prompts</Text>
                    <View style={styles.headerRight} />
                </View>

                {/* Subtitle */}
                <View style={styles.subtitleContainer}>
                    <Text style={styles.subtitle}>
                        Answer up to 3 prompts to help others get to know you better.
                    </Text>
                    <Text style={styles.counter}>
                        {selectedPrompts.length}/{maxPrompts} prompts added
                    </Text>
                </View>

                {/* If currently answering a question */}
                {currentQuestion ? (
                    <View style={styles.answerSection}>
                        <View style={styles.questionBadge}>
                            <Text style={styles.questionBadgeText}>{currentQuestion}</Text>
                        </View>
                        <TextInput
                            style={styles.answerInput}
                            placeholder="Write your answer here..."
                            placeholderTextColor="#999"
                            multiline
                            maxLength={300}
                            value={answerText}
                            onChangeText={setAnswerText}
                            autoFocus
                        />
                        <Text style={styles.charCount}>{answerText.length}/300</Text>
                        <View style={styles.answerButtons}>
                            <TouchableOpacity onPress={handleCancel} style={styles.cancelButton}>
                                <Text style={styles.cancelButtonText}>Cancel</Text>
                            </TouchableOpacity>
                            <TouchableOpacity
                                onPress={handleSaveAnswer}
                                style={[styles.saveButton, !answerText.trim() && styles.saveButtonDisabled]}
                                disabled={!answerText.trim()}
                            >
                                <Text style={styles.saveButtonText}>Save</Text>
                            </TouchableOpacity>
                        </View>
                    </View>
                ) : (
                    <>
                        {/* Saved prompts list */}
                        <FlatList
                            data={selectedPrompts}
                            renderItem={renderPromptCard}
                            keyExtractor={(_, index) => index.toString()}
                            contentContainerStyle={styles.promptsList}
                            ListEmptyComponent={
                                <View style={styles.emptyState}>
                                    <Ionicons name="chatbubble-ellipses-outline" size={48} color="#ccc" />
                                    <Text style={styles.emptyStateText}>No prompts added yet</Text>
                                    <Text style={styles.emptyStateSubtext}>
                                        Tap the button below to add your first prompt
                                    </Text>
                                </View>
                            }
                        />

                        {/* Add prompt button */}
                        {canAddMore && (
                            <TouchableOpacity
                                style={styles.addPromptButton}
                                onPress={() => setShowQuestionPicker(true)}
                            >
                                <Ionicons name="add-circle" size={24} color="#8B5A9C" />
                                <Text style={styles.addPromptButtonText}>Add a prompt</Text>
                            </TouchableOpacity>
                        )}

                        {/* Submit button */}
                        {selectedPrompts.length > 0 && (
                            <View style={styles.submitContainer}>
                                <TouchableOpacity
                                    style={[styles.submitButton, isLoading && styles.submitButtonDisabled]}
                                    onPress={handleSubmitAll}
                                    disabled={isLoading}
                                >
                                    {isLoading ? (
                                        <ActivityIndicator color="#fff" />
                                    ) : (
                                        <Text style={styles.submitButtonText}>Save All Prompts</Text>
                                    )}
                                </TouchableOpacity>
                            </View>
                        )}
                    </>
                )}

                {/* Question Picker Modal */}
                <Modal
                    visible={showQuestionPicker}
                    animationType="slide"
                    presentationStyle="pageSheet"
                >
                    <SafeAreaView style={styles.modalContainer}>
                        <View style={styles.modalHeader}>
                            <TouchableOpacity onPress={() => setShowQuestionPicker(false)}>
                                <Ionicons name="close" size={28} color="#000" />
                            </TouchableOpacity>
                            <Text style={styles.modalTitle}>Choose a prompt</Text>
                            <View style={{ width: 28 }} />
                        </View>
                        <FlatList
                            data={availableQuestions}
                            renderItem={({ item }) => (
                                <TouchableOpacity
                                    style={styles.questionItem}
                                    onPress={() => handleSelectQuestion(item)}
                                >
                                    <Text style={styles.questionItemText}>{item}</Text>
                                    <Ionicons name="chevron-forward" size={20} color="#999" />
                                </TouchableOpacity>
                            )}
                            keyExtractor={(item) => item}
                            contentContainerStyle={styles.questionsList}
                        />
                    </SafeAreaView>
                </Modal>
            </KeyboardAvoidingView>
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#fff",
    },
    keyboardAvoidingView: {
        flex: 1,
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
    promptsList: {
        padding: 20,
        flexGrow: 1,
    },
    promptCard: {
        backgroundColor: "#f8f4fa",
        borderRadius: 16,
        padding: 16,
        marginBottom: 12,
    },
    promptHeader: {
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "flex-start",
        marginBottom: 8,
    },
    promptQuestion: {
        fontSize: 14,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#8B5A9C",
        flex: 1,
    },
    promptActions: {
        flexDirection: "row",
        gap: 8,
    },
    actionButton: {
        padding: 4,
    },
    promptAnswer: {
        fontSize: 16,
        fontFamily: "NunitoSans",
        color: "#333",
        lineHeight: 24,
    },
    emptyState: {
        alignItems: "center",
        justifyContent: "center",
        paddingVertical: 60,
    },
    emptyStateText: {
        fontSize: 18,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#999",
        marginTop: 16,
    },
    emptyStateSubtext: {
        fontSize: 14,
        fontFamily: "NunitoSans",
        color: "#bbb",
        marginTop: 8,
        textAlign: "center",
    },
    addPromptButton: {
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "center",
        gap: 8,
        paddingVertical: 16,
        marginHorizontal: 20,
        borderWidth: 2,
        borderColor: "#8B5A9C",
        borderRadius: 30,
        borderStyle: "dashed",
    },
    addPromptButtonText: {
        fontSize: 16,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#8B5A9C",
    },
    submitContainer: {
        padding: 20,
    },
    submitButton: {
        backgroundColor: "#8B5A9C",
        paddingVertical: 16,
        borderRadius: 30,
        alignItems: "center",
    },
    submitButtonDisabled: {
        opacity: 0.6,
    },
    submitButtonText: {
        fontSize: 16,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#fff",
    },
    // Answer section
    answerSection: {
        flex: 1,
        padding: 20,
    },
    questionBadge: {
        backgroundColor: "#f8f4fa",
        paddingHorizontal: 16,
        paddingVertical: 12,
        borderRadius: 12,
        marginBottom: 16,
    },
    questionBadgeText: {
        fontSize: 16,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#8B5A9C",
    },
    answerInput: {
        flex: 1,
        fontSize: 18,
        fontFamily: "NunitoSans",
        color: "#000",
        textAlignVertical: "top",
        lineHeight: 28,
    },
    charCount: {
        fontSize: 12,
        color: "#999",
        fontFamily: "NunitoSans",
        textAlign: "right",
        marginTop: 8,
    },
    answerButtons: {
        flexDirection: "row",
        justifyContent: "space-between",
        gap: 12,
        marginTop: 16,
    },
    cancelButton: {
        flex: 1,
        paddingVertical: 14,
        borderRadius: 30,
        alignItems: "center",
        borderWidth: 2,
        borderColor: "#ddd",
    },
    cancelButtonText: {
        fontSize: 16,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#666",
    },
    saveButton: {
        flex: 1,
        backgroundColor: "#8B5A9C",
        paddingVertical: 14,
        borderRadius: 30,
        alignItems: "center",
    },
    saveButtonDisabled: {
        backgroundColor: "#ddd",
    },
    saveButtonText: {
        fontSize: 16,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#fff",
    },
    // Modal styles
    modalContainer: {
        flex: 1,
        backgroundColor: "#fff",
    },
    modalHeader: {
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "space-between",
        paddingHorizontal: 20,
        paddingVertical: 16,
        borderBottomWidth: 1,
        borderBottomColor: "#f0f0f0",
    },
    modalTitle: {
        fontSize: 18,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#000",
    },
    questionsList: {
        padding: 20,
    },
    questionItem: {
        flexDirection: "row",
        alignItems: "center",
        justifyContent: "space-between",
        paddingVertical: 16,
        borderBottomWidth: 1,
        borderBottomColor: "#f0f0f0",
    },
    questionItemText: {
        fontSize: 16,
        fontFamily: "NunitoSans",
        color: "#333",
        flex: 1,
    },
});
