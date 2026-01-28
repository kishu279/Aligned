import { updateProfile } from "@/lib/api/endpoints";
import { Ionicons } from "@expo/vector-icons";
import { useRouter } from "expo-router";
import { useRef, useState } from "react";
import {
  ActivityIndicator,
  Alert,
  Animated,
  KeyboardAvoidingView,
  Platform,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  TouchableOpacity,
  View,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";

// Option Picker Component - Minimal Style
const OptionPicker = ({
  options,
  selected,
  onSelect,
  disabled = false,
}: {
  options: string[];
  selected: string;
  onSelect: (value: string) => void;
  disabled?: boolean;
}) => (
  <View style={[styles.optionContainer, disabled && { opacity: 0.5 }]}>
    {options.map((option) => (
      <TouchableOpacity
        key={option}
        style={[
          styles.optionButton,
          selected === option && styles.optionSelected,
        ]}
        onPress={() => !disabled && onSelect(option)}
        activeOpacity={disabled ? 1 : 0.7}
      >
        <Text
          style={[
            styles.optionText,
            selected === option && styles.optionTextSelected,
          ]}
        >
          {option}
        </Text>
      </TouchableOpacity>
    ))}
  </View>
);

export default function ProfileSetupScreen() {
  const router = useRouter();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [step, setStep] = useState(1);
  const scrollViewRef = useRef<ScrollView>(null);

  // Basic Info
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [birthdate, setBirthdate] = useState(""); // YYYY-MM-DD

  // Identity
  const [gender, setGender] = useState("");
  const [pronouns, setPronouns] = useState("");
  const [sexuality, setSexuality] = useState("");

  // About
  const [bio, setBio] = useState("");
  const [height, setHeight] = useState("");
  const [location, setLocation] = useState("");

  // Work & Education
  const [job, setJob] = useState("");
  const [company, setCompany] = useState("");
  const [school, setSchool] = useState("");

  // Background
  const [ethnicity, setEthnicity] = useState("");
  const [religion, setReligion] = useState("");
  const [politics, setPolitics] = useState("");

  // Lifestyle
  const [drinks, setDrinks] = useState("");
  const [smokes, setSmokes] = useState("");

  // Relationship
  const [relationshipType, setRelationshipType] = useState("");
  const [datingIntention, setDatingIntention] = useState("");

  const handleNext = () => {
    if (step < 3) {
      setStep(step + 1);
      scrollViewRef.current?.scrollTo({ y: 0, animated: true });
    } else {
      handleSubmit();
    }
  };

  const handleBack = () => {
    if (step > 1) {
      setStep(step - 1);
    } else {
      router.back();
    }
  };

  const handleSubmit = async () => {
    if (!firstName) return;

    setIsSubmitting(true);
    try {
      const fullName = lastName ? `${firstName} ${lastName}` : firstName;

      let response = await updateProfile({
        name: fullName,
        bio: bio || undefined,
        birthdate: birthdate || undefined,
        pronouns: pronouns || undefined,
        gender: gender || undefined,
        sexuality: sexuality || undefined,
        height: height ? parseInt(height) : undefined,
        location: location || undefined,
        job: job || undefined,
        company: company || undefined,
        school: school || undefined,
        ethnicity: ethnicity || undefined,
        religion: religion || undefined,
        politics: politics || undefined,
        relationship_type: relationshipType || undefined,
        dating_intention: datingIntention || undefined,
        drinks: drinks || undefined,
        smokes: smokes || undefined,
      });

      if (response.status === "success") {
        router.replace("/(tabs)");
      } else {
        router.replace("/auth/profile");
      }
    } catch (error) {
      console.error("Failed to create profile:", error);
      Alert.alert("Error", "Failed to save profile. Please try again.");
    } finally {
      setIsSubmitting(false);
    }
  };

  // Validation for enabling the Next button
  const canProceed = () => {
    if (step === 1) return !!firstName; // Only name is strictly required
    return true;
  };

  return (
    <SafeAreaView style={styles.container}>
      <KeyboardAvoidingView
        behavior={Platform.OS === "ios" ? "padding" : "height"}
        style={styles.keyboardAvoidingView}
      >
        <View style={styles.header}>
          <TouchableOpacity onPress={handleBack} style={styles.backButton}>
            <Ionicons name="chevron-back" size={28} color="#000" />
          </TouchableOpacity>

          {/* Progress Dots */}
          <View style={styles.progressContainer}>
            {[1, 2, 3].map((s) => (
              <View
                key={s}
                style={[
                  styles.progressDot,
                  step >= s && styles.progressDotActive,
                ]}
              />
            ))}
          </View>

          <TouchableOpacity
            onPress={() => router.replace("/(tabs)")}
            disabled={isSubmitting}
            style={{ minWidth: 44, alignItems: "flex-end" }}
          >
            <Text
              style={{
                fontSize: 16,
                fontWeight: "600",
                color: isSubmitting ? "#ccc" : "#000",
              }}
            >
              Skip
            </Text>
          </TouchableOpacity>
        </View>

        <ScrollView
          ref={scrollViewRef}
          style={styles.scrollView}
          contentContainerStyle={styles.scrollContent}
          showsVerticalScrollIndicator={false}
        >
          {step === 1 && (
            <View>
              <Text style={styles.mainTitle}>Who are you?</Text>

              <View style={styles.section}>
                <Text style={styles.sectionTitle}>The Basics</Text>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="First Name *"
                    placeholderTextColor="#ccc"
                    value={firstName}
                    onChangeText={setFirstName}
                  />
                  <View style={styles.underline} />
                </View>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="Last Name"
                    placeholderTextColor="#ccc"
                    value={lastName}
                    onChangeText={setLastName}
                  />
                  <View style={styles.underline} />
                </View>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="Birthdate (YYYY-MM-DD)"
                    placeholderTextColor="#ccc"
                    value={birthdate}
                    onChangeText={setBirthdate}
                  />
                  <View style={styles.underline} />
                </View>
              </View>

              <View style={styles.section}>
                <Text style={styles.sectionTitle}>Identity</Text>
                <Text style={styles.label}>Gender</Text>
                <OptionPicker
                  options={["Man", "Woman", "Non-binary"]}
                  selected={gender}
                  onSelect={setGender}
                />
                <Text style={styles.label}>Pronouns</Text>
                <OptionPicker
                  options={["he/him", "she/her", "they/them"]}
                  selected={pronouns}
                  onSelect={setPronouns}
                />
                <Text style={styles.label}>Sexuality</Text>
                <OptionPicker
                  options={[
                    "Straight",
                    "Gay",
                    "Lesbian",
                    "Bisexual",
                    "Pansexual",
                  ]}
                  selected={sexuality}
                  onSelect={setSexuality}
                />
              </View>
            </View>
          )}

          {step === 2 && (
            <View>
              <Text style={styles.mainTitle}>Tell us more</Text>

              <View style={styles.section}>
                <Text style={styles.sectionTitle}>About You</Text>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={[styles.input, styles.textArea]}
                    placeholder="Bio (a short intro)"
                    placeholderTextColor="#ccc"
                    value={bio}
                    onChangeText={setBio}
                    multiline
                  />
                  <View style={styles.underline} />
                </View>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="Height (cm)"
                    placeholderTextColor="#ccc"
                    value={height}
                    onChangeText={setHeight}
                    keyboardType="numeric"
                  />
                  <View style={styles.underline} />
                </View>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="Location (City)"
                    placeholderTextColor="#ccc"
                    value={location}
                    onChangeText={setLocation}
                  />
                  <View style={styles.underline} />
                </View>
              </View>

              <View style={styles.section}>
                <Text style={styles.sectionTitle}>Work & Education</Text>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="Job Title"
                    placeholderTextColor="#ccc"
                    value={job}
                    onChangeText={setJob}
                  />
                  <View style={styles.underline} />
                </View>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="Company"
                    placeholderTextColor="#ccc"
                    value={company}
                    onChangeText={setCompany}
                  />
                  <View style={styles.underline} />
                </View>
                <View style={styles.inputGroup}>
                  <TextInput
                    style={styles.input}
                    placeholder="School"
                    placeholderTextColor="#ccc"
                    value={school}
                    onChangeText={setSchool}
                  />
                  <View style={styles.underline} />
                </View>
              </View>
            </View>
          )}

          {step === 3 && (
            <View>
              <Text style={styles.mainTitle}>Your details</Text>

              <View style={styles.section}>
                <Text style={styles.sectionTitle}>Background</Text>
                <Text style={styles.label}>Ethnicity</Text>
                <OptionPicker
                  options={[
                    "Asian",
                    "Black",
                    "Hispanic/Latino",
                    "White",
                    "Mixed",
                    "Other",
                  ]}
                  selected={ethnicity}
                  onSelect={setEthnicity}
                />
                <Text style={styles.label}>Religion</Text>
                <OptionPicker
                  options={[
                    "Agnostic",
                    "Atheist",
                    "Buddhist",
                    "Christian",
                    "Hindu",
                    "Jewish",
                    "Muslim",
                    "Spiritual",
                    "Other",
                  ]}
                  selected={religion}
                  onSelect={setReligion}
                />
                <Text style={styles.label}>Politics</Text>
                <OptionPicker
                  options={[
                    "Liberal",
                    "Moderate",
                    "Conservative",
                    "Not Political",
                    "Other",
                  ]}
                  selected={politics}
                  onSelect={setPolitics}
                />
              </View>

              <View style={styles.section}>
                <Text style={styles.sectionTitle}>Lifestyle</Text>
                <Text style={styles.label}>Drinking</Text>
                <OptionPicker
                  options={["Yes", "Socially", "Rarely", "No"]}
                  selected={drinks}
                  onSelect={setDrinks}
                />
                <Text style={styles.label}>Smoking</Text>
                <OptionPicker
                  options={["Yes", "Socially", "No"]}
                  selected={smokes}
                  onSelect={setSmokes}
                />
              </View>
            </View>
          )}

          {/* Spacer for FAB */}
          <View style={{ height: 100 }} />
        </ScrollView>

        {/* FAB */}
        <View style={styles.fabContainer}>
          <TouchableOpacity
            style={[
              styles.fab,
              (!canProceed() || isSubmitting) && styles.fabDisabled,
            ]}
            onPress={handleNext}
            disabled={!canProceed() || isSubmitting}
          >
            {isSubmitting ? (
              <ActivityIndicator color="#fff" />
            ) : (
              <Ionicons
                name={step === 3 ? "checkmark" : "chevron-forward"}
                size={28}
                color={!canProceed() ? "#999" : "#fff"}
              />
            )}
          </TouchableOpacity>
        </View>
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
    paddingHorizontal: 8,
    paddingVertical: 12,
  },
  progressContainer: {
    flexDirection: "row",
    gap: 8,
  },
  progressDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: "#eee",
  },
  progressDotActive: {
    backgroundColor: "#000",
  },
  backButton: {
    padding: 8,
    paddingLeft: 24,
  },
  scrollView: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: 24,
    paddingTop: 10,
  },
  mainTitle: {
    fontSize: 32,
    fontWeight: "900",
    fontFamily: "NunitoSans",
    color: "#000",
    marginBottom: 30,
    lineHeight: 40,
  },
  section: {
    marginBottom: 40,
  },
  sectionTitle: {
    fontSize: 20,
    fontWeight: "700",
    fontFamily: "NunitoSans",
    color: "#000",
    marginBottom: 20,
  },
  inputGroup: {
    marginBottom: 32,
  },
  label: {
    fontSize: 16,
    fontWeight: "600",
    fontFamily: "NunitoSans",
    color: "#000",
    marginTop: 10,
    marginBottom: 12,
  },
  input: {
    fontSize: 24,
    fontWeight: "600",
    fontFamily: "NunitoSans",
    color: "#000",
    paddingVertical: 8,
  },
  textArea: {
    minHeight: 40,
    fontSize: 20,
  },
  underline: {
    height: 1,
    backgroundColor: "#000",
  },
  optionContainer: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 8,
    marginBottom: 20,
  },
  optionButton: {
    paddingHorizontal: 20,
    paddingVertical: 10,
    borderRadius: 24,
    backgroundColor: "#fff",
    borderWidth: 1.5,
    borderColor: "#eee",
  },
  optionSelected: {
    backgroundColor: "#000",
    borderColor: "#000",
  },
  optionText: {
    fontSize: 15,
    fontFamily: "NunitoSans",
    color: "#000",
    fontWeight: "600",
  },
  optionTextSelected: {
    color: "#fff",
  },
  fabContainer: {
    position: "absolute",
    bottom: 40,
    right: 24,
  },
  fab: {
    width: 64,
    height: 64,
    borderRadius: 32,
    backgroundColor: "#000",
    justifyContent: "center",
    alignItems: "center",
    shadowColor: "#000",
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 8,
    elevation: 8,
  },
  fabDisabled: {
    backgroundColor: "#f0f0f0",
    shadowOpacity: 0,
    elevation: 0,
  },
});
