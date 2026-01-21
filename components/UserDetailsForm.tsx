import type { CreateUser as CreateUserType } from "@/lib/api/endpoints";
import { createUser } from "@/lib/api/endpoints";
import { Ionicons } from "@expo/vector-icons";
import React, { useState } from "react";
import {
    ActivityIndicator,
    Alert,
    KeyboardAvoidingView,
    Modal,
    Platform,
    StyleSheet,
    Text,
    TextInput,
    TouchableOpacity,
    View,
} from "react-native";

interface UserDetailsFormProps {
    phone?: string;
    email?: string;
    onSuccess: () => void;
    onBack?: () => void;
}

export default function UserDetailsForm({ phone, email, onSuccess, onBack }: UserDetailsFormProps) {
    // Determine what we need to collect
    const hasPhone = !!phone;
    const hasEmail = !!email;
    const collectingEmail = hasPhone && !hasEmail;
    const collectingPhone = hasEmail && !hasPhone;

    const [inputValue, setInputValue] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const [showSuccessModal, setShowSuccessModal] = useState(false);
    const [showFailureModal, setShowFailureModal] = useState(false);
    const [errorMessage, setErrorMessage] = useState("");

    const handleSubmit = async () => {
        if (!inputValue.trim()) {
            Alert.alert("Error", collectingEmail ? "Email is required" : "Phone number is required");
            return;
        }

        let userData: CreateUserType | null = null;

        // Basic validation
        if (collectingEmail) {
            const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
            if (!emailRegex.test(inputValue)) {
                Alert.alert("Error", "Please enter a valid email address");
                return;
            }
            userData = {
                email: inputValue,
                phone: phone!,
            };
        } else if (collectingPhone) {
            // Simple phone validation
            if (inputValue.length < 10) {
                Alert.alert("Error", "Please enter a valid phone number");
                return;
            }
            userData = {
                email: email!,
                phone: inputValue,
            };
        }

        if (!userData) {
            Alert.alert("Error", "Failed to create user");
            return;
        }

        setIsLoading(true);
        try {
            console.log('[UserDetailsForm] Creating user:', userData);
            let response = await createUser(userData);

            if (response.status === "error") {
                setErrorMessage(response.message || "Failed to create account");
                setShowFailureModal(true);
                setTimeout(() => setShowFailureModal(false), 3000);
                return;
            }

            // Show success popup
            setShowSuccessModal(true);

            // Callback after a brief delay
            setTimeout(() => {
                setShowSuccessModal(false);
                onSuccess();
            }, 2000);
        } catch (error: any) {
            Alert.alert("Error", error.message || "Failed to save details");
        } finally {
            setIsLoading(false);
        }
    };

    // Dynamic content based on what we're collecting
    const icon = collectingEmail ? "mail-outline" : "call-outline";
    const title = collectingEmail
        ? "What's your email\naddress?"
        : "What's your phone\nnumber?";
    const placeholder = collectingEmail ? "your@email.com" : "";
    const keyboardType = collectingEmail ? "email-address" : "phone-pad";
    const footerText = collectingEmail
        ? "We'll use this to send you important updates and help you recover your account."
        : "We'll send you a verification code to confirm your number.";

    const isValid = inputValue.trim().length > 0;

    return (
        <KeyboardAvoidingView
            behavior={Platform.OS === "ios" ? "padding" : "height"}
            style={styles.keyboardAvoidingView}
        >
            {onBack && (
                <TouchableOpacity onPress={onBack} style={styles.backButton}>
                    <Ionicons name="chevron-back" size={28} color="#000" />
                </TouchableOpacity>
            )}

            <View style={styles.content}>
                {/* Header Icon */}
                <View style={styles.iconContainer}>
                    <Ionicons name={icon} size={32} color="#000" />
                </View>

                {/* Title */}
                <Text style={styles.title}>{title}</Text>

                {/* Show what we already have */}
                {hasPhone && (
                    <View style={styles.existingInfo}>
                        <Ionicons name="checkmark-circle" size={16} color="#4CAF50" />
                        <Text style={styles.existingText}>Phone: {phone}</Text>
                    </View>
                )}
                {hasEmail && (
                    <View style={styles.existingInfo}>
                        <Ionicons name="checkmark-circle" size={16} color="#4CAF50" />
                        <Text style={styles.existingText}>Email: {email}</Text>
                    </View>
                )}

                {/* Input */}
                {collectingEmail ? (
                    <View style={styles.inputContainer}>
                        <TextInput
                            style={styles.input}
                            placeholder={placeholder}
                            placeholderTextColor="#999"
                            keyboardType={keyboardType}
                            autoCapitalize="none"
                            autoFocus
                            value={inputValue}
                            onChangeText={setInputValue}
                            selectionColor="#8B5A9C"
                            editable={!isLoading}
                        />
                        <View style={styles.underline} />
                    </View>
                ) : collectingPhone ? (
                    <>
                        <View style={styles.inputRow}>
                            {/* Country Code */}
                            <View style={styles.countryCodeContainer}>
                                <Text style={styles.flag}>🇮🇳</Text>
                                <Text style={styles.countryCodeText}>+91</Text>
                                <Ionicons name="chevron-down" size={16} color="#000" />
                            </View>
                            <View style={styles.divider} />
                            <TextInput
                                style={styles.phoneInput}
                                placeholder=""
                                keyboardType="phone-pad"
                                autoFocus
                                value={inputValue}
                                onChangeText={setInputValue}
                                selectionColor="#8B5A9C"
                                editable={!isLoading}
                            />
                        </View>
                        <View style={styles.underline} />
                    </>
                ) : (
                    <Text style={styles.errorText}>Missing parameters</Text>
                )}

                {/* Footer Text */}
                <Text style={styles.footerText}>{footerText}</Text>
            </View>

            {/* Floating Action Button */}
            <View style={styles.fabContainer}>
                <TouchableOpacity
                    style={[styles.fab, (!isValid || isLoading) && styles.fabDisabled]}
                    onPress={handleSubmit}
                    disabled={!isValid || isLoading}
                >
                    {isLoading ? (
                        <ActivityIndicator color="#999" />
                    ) : (
                        <Ionicons name="chevron-forward" size={28} color={!isValid ? "#999" : "#fff"} />
                    )}
                </TouchableOpacity>
            </View>

            {/* Success Modal */}
            <Modal
                visible={showSuccessModal}
                transparent={true}
                animationType="fade"
            >
                <View style={styles.modalOverlay}>
                    <View style={styles.modalContent}>
                        <View style={styles.successIconContainer}>
                            <Ionicons name="checkmark-circle" size={64} color="#4CAF50" />
                        </View>
                        <Text style={styles.modalTitle}>Account Created!</Text>
                        <Text style={styles.modalMessage}>
                            Welcome aboard! Your account has been successfully created.
                        </Text>
                    </View>
                </View>
            </Modal>

            {/* Failure Modal */}
            <Modal
                visible={showFailureModal}
                transparent={true}
                animationType="fade"
            >
                <View style={styles.modalOverlay}>
                    <View style={styles.modalContent}>
                        <View style={styles.successIconContainer}>
                            <Ionicons name="close-circle" size={64} color="#f44336" />
                        </View>
                        <Text style={styles.modalTitle}>Account Creation Failed</Text>
                        <Text style={styles.modalMessage}>
                            {errorMessage}
                        </Text>
                    </View>
                </View>
            </Modal>
        </KeyboardAvoidingView>
    );
}

const styles = StyleSheet.create({
    keyboardAvoidingView: {
        flex: 1,
    },
    backButton: {
        padding: 16,
    },
    content: {
        flex: 1,
        paddingHorizontal: 24,
        paddingTop: 40,
    },
    iconContainer: {
        width: 60,
        height: 60,
        borderRadius: 30,
        borderWidth: 2,
        borderColor: "#000",
        justifyContent: "center",
        alignItems: "center",
        marginBottom: 24,
    },
    title: {
        fontSize: 32,
        fontWeight: "900",
        fontFamily: "NunitoSans",
        color: "#000",
        marginBottom: 16,
        lineHeight: 40,
    },
    existingInfo: {
        flexDirection: "row",
        alignItems: "center",
        gap: 8,
        marginBottom: 24,
        paddingVertical: 8,
        paddingHorizontal: 12,
        backgroundColor: "#f0f9f0",
        borderRadius: 8,
        alignSelf: "flex-start",
    },
    existingText: {
        fontSize: 14,
        color: "#333",
        fontFamily: "NunitoSans",
    },
    inputContainer: {
        marginBottom: 16,
    },
    input: {
        fontSize: 20,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#000",
        paddingVertical: 8,
    },
    inputRow: {
        flexDirection: "row",
        alignItems: "center",
        marginBottom: 8,
    },
    countryCodeContainer: {
        flexDirection: "row",
        alignItems: "center",
        gap: 8,
        marginRight: 16,
    },
    flag: {
        fontSize: 24,
    },
    countryCodeText: {
        fontSize: 20,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#000",
    },
    divider: {
        width: 1,
        height: 24,
        backgroundColor: "#ccc",
        marginRight: 16,
    },
    phoneInput: {
        flex: 1,
        fontSize: 20,
        fontWeight: "600",
        fontFamily: "NunitoSans",
        color: "#000",
        padding: 0,
    },
    underline: {
        height: 1,
        backgroundColor: "#000",
        marginBottom: 16,
    },
    footerText: {
        fontSize: 14,
        color: "#888",
        fontFamily: "NunitoSans",
        lineHeight: 20,
    },
    errorText: {
        fontSize: 16,
        color: "#f44336",
        fontFamily: "NunitoSans",
    },
    fabContainer: {
        padding: 24,
        alignItems: "flex-end",
    },
    fab: {
        width: 56,
        height: 56,
        borderRadius: 28,
        backgroundColor: "#000",
        justifyContent: "center",
        alignItems: "center",
        shadowColor: "#000",
        shadowOffset: { width: 0, height: 2 },
        shadowOpacity: 0.2,
        shadowRadius: 4,
        elevation: 5,
    },
    fabDisabled: {
        backgroundColor: "#f0f0f0",
        shadowOpacity: 0,
        elevation: 0,
    },
    modalOverlay: {
        flex: 1,
        backgroundColor: "rgba(0, 0, 0, 0.5)",
        justifyContent: "center",
        alignItems: "center",
    },
    modalContent: {
        backgroundColor: "#fff",
        borderRadius: 20,
        padding: 32,
        alignItems: "center",
        marginHorizontal: 40,
        shadowColor: "#000",
        shadowOffset: { width: 0, height: 4 },
        shadowOpacity: 0.25,
        shadowRadius: 8,
        elevation: 10,
    },
    successIconContainer: {
        marginBottom: 16,
    },
    modalTitle: {
        fontSize: 24,
        fontWeight: "700",
        fontFamily: "NunitoSans",
        color: "#000",
        marginBottom: 8,
        textAlign: "center",
    },
    modalMessage: {
        fontSize: 16,
        color: "#666",
        fontFamily: "NunitoSans",
        textAlign: "center",
        lineHeight: 22,
    },
});
