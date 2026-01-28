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
  Alert,
  Modal,
  TextInput,
  Platform,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import {
  getPreferences,
  Preferences,
  getMyProfile,
  ProfileDetails,
  updatePreferences,
} from "../lib/api/endpoints";
import {
  GENDER_OPTIONS,
  ETHNICITY_OPTIONS,
  RELIGION_OPTIONS,
  DISTANCE_OPTIONS,
  PREFERENCE_FIELDS,
} from "../constants/preferences";

// Preference field keys that can be edited
type PreferenceKey =
  | "genderPreference"
  | "distanceMax"
  | "ageRange"
  | "ethnicityPreference"
  | "religionPreference";

export default function DatingPreferences() {
  const router = useRouter();
  const [preferences, setPreferences] = useState<Preferences | null>(null);
  const [profile, setProfile] = useState<ProfileDetails | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [editModalVisible, setEditModalVisible] = useState(false);
  const [editingField, setEditingField] = useState<string>("");
  const [fieldKey, setFieldKey] = useState<PreferenceKey | "">("");

  // For multi-select fields
  const [selectedOptions, setSelectedOptions] = useState<string[]>([]);

  // For single-select fields (distance)
  const [selectedDistance, setSelectedDistance] = useState<number | null>(null);

  // For age range
  const [ageMin, setAgeMin] = useState<string>("");
  const [ageMax, setAgeMax] = useState<string>("");

  // Track if there are unsaved changes
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false);
  const [originalPreferences, setOriginalPreferences] =
    useState<Preferences | null>(null);

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    try {
      setLoading(true);
      const [prefsResult, profileResult] = await Promise.allSettled([
        getPreferences(),
        getMyProfile(),
      ]);

      if (prefsResult.status === "fulfilled") {
        setPreferences(prefsResult.value);
        setOriginalPreferences(prefsResult.value);
      }

      if (profileResult.status === "fulfilled") {
        setProfile(profileResult.value.details || null);
      }
    } catch (error) {
      console.error("Failed to fetch data:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleEditPress = (field: string, key: PreferenceKey) => {
    setEditingField(field);
    setFieldKey(key);

    // Initialize values based on the field type
    switch (key) {
      case "genderPreference":
        setSelectedOptions(preferences?.genderPreference || []);
        break;
      case "ethnicityPreference":
        setSelectedOptions(preferences?.ethnicityPreference || []);
        break;
      case "religionPreference":
        setSelectedOptions(preferences?.religionPreference || []);
        break;
      case "distanceMax":
        setSelectedDistance(preferences?.distanceMax || null);
        break;
      case "ageRange":
        setAgeMin(preferences?.ageRange?.min?.toString() || "");
        setAgeMax(preferences?.ageRange?.max?.toString() || "");
        break;
    }

    setEditModalVisible(true);
  };

  const toggleOption = (option: string) => {
    setSelectedOptions((prev) => {
      if (prev.includes(option)) {
        return prev.filter((o) => o !== option);
      } else {
        return [...prev, option];
      }
    });
  };

  // Save to local state only (modal Save button)
  const handleSaveToState = () => {
    let updatedPrefs: Partial<Preferences> = {};

    switch (fieldKey) {
      case "genderPreference":
        if (selectedOptions.length === 0) {
          Alert.alert("Error", "Please select at least one option");
          return;
        }
        updatedPrefs.genderPreference = selectedOptions;
        break;
      case "ethnicityPreference":
        if (selectedOptions.length === 0) {
          Alert.alert("Error", "Please select at least one option");
          return;
        }
        updatedPrefs.ethnicityPreference = selectedOptions;
        break;
      case "religionPreference":
        if (selectedOptions.length === 0) {
          Alert.alert("Error", "Please select at least one option");
          return;
        }
        updatedPrefs.religionPreference = selectedOptions;
        break;
      case "distanceMax":
        if (!selectedDistance) {
          Alert.alert("Error", "Please select a distance");
          return;
        }
        updatedPrefs.distanceMax = selectedDistance;
        break;
      case "ageRange":
        const min = parseInt(ageMin, 10);
        const max = parseInt(ageMax, 10);
        if (isNaN(min) || isNaN(max)) {
          Alert.alert("Error", "Please enter valid ages");
          return;
        }
        if (min < 18 || max > 100) {
          Alert.alert("Error", "Age must be between 18 and 100");
          return;
        }
        if (min > max) {
          Alert.alert("Error", "Minimum age must be less than maximum age");
          return;
        }
        updatedPrefs.ageRange = { min, max };
        break;
      default:
        Alert.alert("Error", "Unknown field");
        return;
    }

    // Save to local state
    setPreferences(
      (prev) =>
        ({
          ...prev,
          ...updatedPrefs,
        }) as Preferences,
    );
    setHasUnsavedChanges(true);
    setEditModalVisible(false);
  };

  // Build summary of changes for confirmation
  const buildChangesSummary = (): string => {
    if (!preferences || !originalPreferences) return "";

    const changes: string[] = [];

    if (
      JSON.stringify(preferences.genderPreference) !==
      JSON.stringify(originalPreferences.genderPreference)
    ) {
      changes.push(
        `• Gender: ${preferences.genderPreference?.join(", ") || "Not set"}`,
      );
    }
    if (preferences.distanceMax !== originalPreferences.distanceMax) {
      const dist =
        preferences.distanceMax === 9999
          ? "Anywhere"
          : `${preferences.distanceMax} km`;
      changes.push(`• Distance: ${dist}`);
    }
    if (
      JSON.stringify(preferences.ageRange) !==
      JSON.stringify(originalPreferences.ageRange)
    ) {
      changes.push(
        `• Age Range: ${preferences.ageRange?.min} - ${preferences.ageRange?.max}`,
      );
    }
    if (
      JSON.stringify(preferences.ethnicityPreference) !==
      JSON.stringify(originalPreferences.ethnicityPreference)
    ) {
      changes.push(
        `• Ethnicity: ${preferences.ethnicityPreference?.join(", ") || "Not set"}`,
      );
    }
    if (
      JSON.stringify(preferences.religionPreference) !==
      JSON.stringify(originalPreferences.religionPreference)
    ) {
      changes.push(
        `• Religion: ${preferences.religionPreference?.join(", ") || "Not set"}`,
      );
    }

    return changes.join("\n");
  };

  // Send API request (Update button on main page)
  const handleUpdatePreferences = () => {
    if (!preferences) return;

    const changesSummary = buildChangesSummary();

    Alert.alert(
      "Confirm Update",
      `Are you sure you want to update your preferences?\n\nChanges:\n${changesSummary}`,
      [
        {
          text: "Cancel",
          style: "cancel",
        },
        {
          text: "Update",
          onPress: () => sendUpdateToAPI(),
        },
      ],
    );
  };

  const sendUpdateToAPI = async () => {
    if (!preferences) return;

    setSaving(true);
    try {
      const response = await updatePreferences(preferences);

      if (response.status === "success") {
        setOriginalPreferences(preferences);
        setHasUnsavedChanges(false);
        Alert.alert("Success", "Preferences updated successfully!");
      } else {
        Alert.alert(
          "Error",
          response.message || "Failed to update preferences",
        );
      }
    } catch (error) {
      console.error("Failed to update preferences:", error);
      Alert.alert("Error", "Failed to update preferences. Please try again.");
    } finally {
      setSaving(false);
    }
  };

  const getOptionsForField = (): readonly string[] => {
    switch (fieldKey) {
      case "genderPreference":
        return GENDER_OPTIONS;
      case "ethnicityPreference":
        return ETHNICITY_OPTIONS;
      case "religionPreference":
        return RELIGION_OPTIONS;
      default:
        return [];
    }
  };

  const isMultiSelectField = () => {
    return [
      "genderPreference",
      "ethnicityPreference",
      "religionPreference",
    ].includes(fieldKey);
  };

  const isDistanceField = () => fieldKey === "distanceMax";
  const isAgeRangeField = () => fieldKey === "ageRange";

  // Render chip for multi-select
  const renderChip = (option: string, isSelected: boolean) => (
    <TouchableOpacity
      key={option}
      style={[styles.chip, isSelected && styles.chipSelected]}
      onPress={() => toggleOption(option)}
      activeOpacity={0.7}
    >
      <Text style={[styles.chipText, isSelected && styles.chipTextSelected]}>
        {option}
      </Text>
      {isSelected && (
        <Ionicons
          name="checkmark-circle"
          size={18}
          color="#fff"
          style={{ marginLeft: 4 }}
        />
      )}
    </TouchableOpacity>
  );

  // Render distance option
  const renderDistanceOption = (option: { label: string; value: number }) => (
    <TouchableOpacity
      key={option.value}
      style={[
        styles.distanceOption,
        selectedDistance === option.value && styles.distanceOptionSelected,
      ]}
      onPress={() => setSelectedDistance(option.value)}
      activeOpacity={0.7}
    >
      <Text
        style={[
          styles.distanceOptionText,
          selectedDistance === option.value &&
            styles.distanceOptionTextSelected,
        ]}
      >
        {option.label}
      </Text>
    </TouchableOpacity>
  );

  const PreferenceItem = ({
    title,
    value,
    isLocked,
    isDealbreaker,
    isMissing,
    onPress,
    isEditable = false,
  }: {
    title: string;
    value: string;
    isLocked?: boolean;
    isDealbreaker?: boolean;
    isMissing?: boolean;
    onPress?: () => void;
    isEditable?: boolean;
  }) => {
    const content = (
      <View
        style={[
          styles.itemContainer,
          isMissing && styles.missingFieldContainer,
        ]}
      >
        <View style={styles.itemContent}>
          <View style={styles.itemHeader}>
            <Text style={styles.itemTitle}>{title}</Text>
            <View style={styles.badgeContainer}>
              {isMissing && (
                <View style={styles.missingBadge}>
                  <Ionicons name="alert-circle" size={14} color="#D24E4E" />
                  <Text style={styles.missingText}>Missing</Text>
                </View>
              )}
              {isDealbreaker && (
                <Text style={styles.dealbreakerText}>Dealbreaker</Text>
              )}
            </View>
          </View>
          <Text
            style={[
              styles.itemValue,
              isLocked && !isEditable && styles.lockedText,
              isMissing && styles.missingValue,
            ]}
          >
            {value}
          </Text>
        </View>
        <View style={styles.rightIcons}>
          {isEditable && (
            <Ionicons name="chevron-forward" size={20} color="#7D4ca2" />
          )}
          {isLocked && !isEditable && (
            <Ionicons name="lock-closed-outline" size={20} color="#D24E4E" />
          )}
        </View>
      </View>
    );

    if (onPress && isEditable) {
      return (
        <TouchableOpacity onPress={onPress} activeOpacity={0.7}>
          {content}
        </TouchableOpacity>
      );
    }
    return content;
  };

  return (
    <View style={styles.container}>
      <SafeAreaView style={styles.safeArea} edges={["top"]}>
        {/* Header */}
        <View style={styles.header}>
          <TouchableOpacity
            onPress={() => router.back()}
            style={styles.backButton}
          >
            <Ionicons name="chevron-back" size={28} color="#000" />
          </TouchableOpacity>
          <Text style={styles.headerTitle}>Dating Preferences</Text>
          <View style={{ width: 28 }} />
        </View>

        <ScrollView contentContainerStyle={styles.scrollContent}>
          {loading ? (
            <View style={styles.loadingContainer}>
              <ActivityIndicator size="large" color="#7D4ca2" />
            </View>
          ) : (
            <>
              {/* Member Preferences */}
              <Text style={styles.sectionTitle}>Member Preferences</Text>
              <Text style={styles.sectionSubtitle}>
                Tap to edit your preferences
              </Text>

              <PreferenceItem
                title="I'm interested in"
                value={
                  preferences?.genderPreference &&
                  preferences.genderPreference.length > 0
                    ? preferences.genderPreference.join(", ")
                    : "Not set"
                }
                isMissing={
                  !preferences?.genderPreference ||
                  preferences.genderPreference.length === 0
                }
                isEditable
                onPress={() =>
                  handleEditPress("Gender Preference", "genderPreference")
                }
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="My neighborhood"
                value={profile?.location || "Not set"}
                isMissing={!profile?.location}
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Maximum distance"
                value={
                  preferences?.distanceMax
                    ? preferences.distanceMax === 9999
                      ? "Anywhere"
                      : `${preferences.distanceMax} km`
                    : "Not set"
                }
                isMissing={!preferences?.distanceMax}
                isEditable
                onPress={() =>
                  handleEditPress("Maximum Distance", "distanceMax")
                }
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Age range"
                value={
                  preferences?.ageRange
                    ? `${preferences.ageRange.min} - ${preferences.ageRange.max}`
                    : "Not set"
                }
                isDealbreaker
                isMissing={!preferences?.ageRange}
                isEditable
                onPress={() => handleEditPress("Age Range", "ageRange")}
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Ethnicity preference"
                value={
                  preferences?.ethnicityPreference &&
                  preferences.ethnicityPreference.length > 0
                    ? preferences.ethnicityPreference.join(", ")
                    : "Not set"
                }
                isMissing={
                  !preferences?.ethnicityPreference ||
                  preferences.ethnicityPreference.length === 0
                }
                isEditable
                onPress={() =>
                  handleEditPress("Ethnicity Preference", "ethnicityPreference")
                }
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Religion preference"
                value={
                  preferences?.religionPreference &&
                  preferences.religionPreference.length > 0
                    ? preferences.religionPreference.join(", ")
                    : "Not set"
                }
                isMissing={
                  !preferences?.religionPreference ||
                  preferences.religionPreference.length === 0
                }
                isEditable
                onPress={() =>
                  handleEditPress("Religion Preference", "religionPreference")
                }
              />
              <View style={styles.divider} />

              {/* Profile Info Section */}
              <Text style={[styles.sectionTitle, { marginTop: 32 }]}>
                Profile Info
              </Text>

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
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Dating Intentions"
                value={profile?.dating_intention || "Not set"}
                isMissing={!profile?.dating_intention}
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Relationship Type"
                value={profile?.relationship_type || "Not set"}
                isMissing={!profile?.relationship_type}
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Smoking"
                value={profile?.smokes || "Not set"}
                isMissing={!profile?.smokes}
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Drinking"
                value={profile?.drinks || "Not set"}
                isMissing={!profile?.drinks}
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Politics"
                value={profile?.politics || "Not set"}
                isMissing={!profile?.politics}
                isLocked
              />
              <View style={styles.divider} />

              <PreferenceItem
                title="Education"
                value={profile?.school || "Not set"}
                isMissing={!profile?.school}
                isLocked
              />
              <View style={styles.divider} />

              <View style={{ height: 100 }} />
            </>
          )}
        </ScrollView>

        {/* Update Button - Fixed at bottom */}
        {hasUnsavedChanges && !loading && (
          <View style={styles.updateButtonContainer}>
            <TouchableOpacity
              style={[
                styles.updateButton,
                saving && styles.updateButtonDisabled,
              ]}
              onPress={handleUpdatePreferences}
              disabled={saving}
            >
              {saving ? (
                <ActivityIndicator size="small" color="#fff" />
              ) : (
                <Text style={styles.updateButtonText}>Update Preferences</Text>
              )}
            </TouchableOpacity>
          </View>
        )}

        {/* Edit Modal */}
        <Modal
          visible={editModalVisible}
          transparent
          animationType="slide"
          onRequestClose={() => setEditModalVisible(false)}
        >
          <View style={styles.modalOverlay}>
            <View style={styles.modalContent}>
              <View style={styles.modalHeader}>
                <Text style={styles.modalTitle}>Edit {editingField}</Text>
                <TouchableOpacity
                  onPress={() => setEditModalVisible(false)}
                  style={styles.closeButton}
                >
                  <Ionicons name="close" size={24} color="#000" />
                </TouchableOpacity>
              </View>

              <ScrollView style={styles.modalScrollContent}>
                {/* Multi-select chips for gender, ethnicity, religion */}
                {isMultiSelectField() && (
                  <>
                    <Text style={styles.modalSubtitle}>
                      Select all that apply
                    </Text>
                    <View style={styles.chipsContainer}>
                      {getOptionsForField().map((option) =>
                        renderChip(option, selectedOptions.includes(option)),
                      )}
                    </View>
                  </>
                )}

                {/* Distance options */}
                {isDistanceField() && (
                  <>
                    <Text style={styles.modalSubtitle}>
                      Select maximum distance
                    </Text>
                    <View style={styles.distanceOptionsContainer}>
                      {DISTANCE_OPTIONS.map((option) =>
                        renderDistanceOption(option),
                      )}
                    </View>
                  </>
                )}

                {/* Age range inputs */}
                {isAgeRangeField() && (
                  <>
                    <Text style={styles.modalSubtitle}>
                      Enter your preferred age range
                    </Text>
                    <View style={styles.ageRangeContainer}>
                      <View style={styles.ageInputWrapper}>
                        <Text style={styles.ageLabel}>Min Age</Text>
                        <TextInput
                          style={styles.ageInput}
                          value={ageMin}
                          onChangeText={setAgeMin}
                          placeholder="18"
                          placeholderTextColor="#999"
                          keyboardType="number-pad"
                          maxLength={3}
                        />
                      </View>
                      <Text style={styles.ageDash}>—</Text>
                      <View style={styles.ageInputWrapper}>
                        <Text style={styles.ageLabel}>Max Age</Text>
                        <TextInput
                          style={styles.ageInput}
                          value={ageMax}
                          onChangeText={setAgeMax}
                          placeholder="100"
                          placeholderTextColor="#999"
                          keyboardType="number-pad"
                          maxLength={3}
                        />
                      </View>
                    </View>
                    <Text style={styles.modalHint}>
                      Age must be between 18 and 100
                    </Text>
                  </>
                )}
              </ScrollView>

              <View style={styles.modalButtons}>
                <TouchableOpacity
                  style={styles.modalCancelButton}
                  onPress={() => setEditModalVisible(false)}
                  disabled={saving}
                >
                  <Text style={styles.modalCancelText}>Cancel</Text>
                </TouchableOpacity>
                <TouchableOpacity
                  style={styles.modalSaveButton}
                  onPress={handleSaveToState}
                >
                  <Text style={styles.modalSaveText}>Save</Text>
                </TouchableOpacity>
              </View>
            </View>
          </View>
        </Modal>
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
  sectionTitle: {
    fontSize: 16,
    fontWeight: "900",
    color: "#ccc",
    fontFamily: "NunitoSans",
    paddingHorizontal: 20,
    marginBottom: 4,
  },
  sectionSubtitle: {
    fontSize: 14,
    color: "#7D4ca2",
    fontWeight: "700",
    fontFamily: "NunitoSans",
    paddingHorizontal: 20,
    marginBottom: 16,
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
  badgeContainer: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
  },
  dealbreakerText: {
    fontSize: 12,
    color: "#ccc",
    fontFamily: "NunitoSans",
    fontWeight: "600",
  },
  itemValue: {
    fontSize: 14,
    color: "#000",
    fontFamily: "NunitoSans",
  },
  lockedText: {
    color: "#ccc",
  },
  divider: {
    height: 1,
    backgroundColor: "#f0f0f0",
    marginLeft: 20,
  },
  rightIcons: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
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
  // Modal styles
  modalOverlay: {
    flex: 1,
    backgroundColor: "rgba(0, 0, 0, 0.5)",
    justifyContent: "flex-end",
  },
  modalContent: {
    backgroundColor: "#fff",
    borderTopLeftRadius: 20,
    borderTopRightRadius: 20,
    padding: 20,
    paddingBottom: Platform.OS === "ios" ? 40 : 20,
    maxHeight: "80%",
  },
  modalScrollContent: {
    maxHeight: 400,
  },
  modalHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    marginBottom: 16,
  },
  modalTitle: {
    fontSize: 20,
    fontWeight: "900",
    color: "#000",
    fontFamily: "NunitoSans",
  },
  modalSubtitle: {
    fontSize: 14,
    color: "#666",
    fontFamily: "NunitoSans",
    marginBottom: 16,
  },
  closeButton: {
    padding: 4,
  },
  modalHint: {
    fontSize: 13,
    color: "#666",
    fontFamily: "NunitoSans",
    marginTop: 8,
    marginBottom: 20,
  },
  modalButtons: {
    flexDirection: "row",
    gap: 12,
    marginTop: 20,
  },
  modalCancelButton: {
    flex: 1,
    padding: 16,
    borderRadius: 25,
    borderWidth: 1,
    borderColor: "#E0E0E0",
    alignItems: "center",
  },
  modalCancelText: {
    fontSize: 16,
    fontWeight: "700",
    color: "#666",
    fontFamily: "NunitoSans",
  },
  modalSaveButton: {
    flex: 1,
    padding: 16,
    borderRadius: 25,
    backgroundColor: "#7D4ca2",
    alignItems: "center",
  },
  modalSaveButtonDisabled: {
    backgroundColor: "#B99CC9",
  },
  modalSaveText: {
    fontSize: 16,
    fontWeight: "900",
    color: "#fff",
    fontFamily: "NunitoSans",
  },
  // Chip styles for multi-select
  chipsContainer: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 10,
    marginBottom: 20,
  },
  chip: {
    flexDirection: "row",
    alignItems: "center",
    paddingVertical: 10,
    paddingHorizontal: 16,
    borderRadius: 20,
    backgroundColor: "#F5F5F5",
    borderWidth: 1,
    borderColor: "#E0E0E0",
  },
  chipSelected: {
    backgroundColor: "#7D4ca2",
    borderColor: "#7D4ca2",
  },
  chipText: {
    fontSize: 14,
    fontWeight: "600",
    color: "#333",
    fontFamily: "NunitoSans",
  },
  chipTextSelected: {
    color: "#fff",
  },
  // Distance options styles
  distanceOptionsContainer: {
    gap: 10,
    marginBottom: 20,
  },
  distanceOption: {
    padding: 16,
    borderRadius: 12,
    backgroundColor: "#F5F5F5",
    borderWidth: 1,
    borderColor: "#E0E0E0",
  },
  distanceOptionSelected: {
    backgroundColor: "#7D4ca2",
    borderColor: "#7D4ca2",
  },
  distanceOptionText: {
    fontSize: 16,
    fontWeight: "600",
    color: "#333",
    fontFamily: "NunitoSans",
    textAlign: "center",
  },
  distanceOptionTextSelected: {
    color: "#fff",
  },
  // Age range styles
  ageRangeContainer: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    gap: 16,
    marginBottom: 10,
  },
  ageInputWrapper: {
    flex: 1,
    alignItems: "center",
  },
  ageLabel: {
    fontSize: 13,
    color: "#666",
    fontFamily: "NunitoSans",
    marginBottom: 8,
  },
  ageInput: {
    width: "100%",
    padding: 16,
    borderRadius: 12,
    backgroundColor: "#F5F5F5",
    borderWidth: 1,
    borderColor: "#E0E0E0",
    fontSize: 18,
    fontWeight: "700",
    fontFamily: "NunitoSans",
    textAlign: "center",
    color: "#000",
  },
  ageDash: {
    fontSize: 24,
    color: "#999",
    marginTop: 20,
  },
  // Update button styles
  updateButtonContainer: {
    position: "absolute",
    bottom: 0,
    left: 0,
    right: 0,
    backgroundColor: "#fff",
    paddingHorizontal: 20,
    paddingVertical: 16,
    borderTopWidth: 1,
    borderTopColor: "#f0f0f0",
    paddingBottom: Platform.OS === "ios" ? 34 : 16,
  },
  updateButton: {
    backgroundColor: "#7D4ca2",
    paddingVertical: 16,
    borderRadius: 25,
    alignItems: "center",
  },
  updateButtonDisabled: {
    backgroundColor: "#B99CC9",
  },
  updateButtonText: {
    fontSize: 16,
    fontWeight: "900",
    color: "#fff",
    fontFamily: "NunitoSans",
  },
});
