/**
 * Preference Options Constants
 *
 * This file contains all the predefined options for dating preferences.
 * These values match the backend seed data (seed.rs).
 * Easy to modify and extend in the future.
 */

// ============ Gender Options (for "I'm interested in") ============
// Note: Backend uses singular form ("Woman", "Man")
export const GENDER_OPTIONS = [
  "Woman",
  "Man",
  "Non-binary",
  "Everyone",
] as const;

export type GenderOption = (typeof GENDER_OPTIONS)[number];

// ============ Ethnicity Options ============
// Matches values from seed.rs profiles
export const ETHNICITY_OPTIONS = [
  "Asian",
  "South Asian",
  "Black",
  "White",
  "Latino",
  "Latina",
  "Middle Eastern",
  "Native American",
  "Pacific Islander",
  "Mixed",
  "Other",
  "Open to all",
] as const;

export type EthnicityOption = (typeof ETHNICITY_OPTIONS)[number];

// ============ Religion Options ============
// Matches values from seed.rs profiles
export const RELIGION_OPTIONS = [
  "Christian",
  "Catholic",
  "Muslim",
  "Hindu",
  "Buddhist",
  "Jewish",
  "Sikh",
  "Agnostic",
  "Atheist",
  "Spiritual",
  "Other",
  "Open to all",
] as const;

export type ReligionOption = (typeof RELIGION_OPTIONS)[number];

// ============ Distance Options (in km) ============
export const DISTANCE_OPTIONS = [
  { label: "5 km", value: 5 },
  { label: "10 km", value: 10 },
  { label: "25 km", value: 25 },
  { label: "50 km", value: 50 },
  { label: "100 km", value: 100 },
  { label: "150 km", value: 150 },
  { label: "Anywhere", value: 9999 },
] as const;

export type DistanceOption = (typeof DISTANCE_OPTIONS)[number];

// ============ Age Range Presets ============
export const AGE_RANGE_PRESETS = [
  { label: "18-24", min: 18, max: 24 },
  { label: "25-30", min: 25, max: 30 },
  { label: "31-40", min: 31, max: 40 },
  { label: "41-50", min: 41, max: 50 },
  { label: "50+", min: 50, max: 100 },
] as const;

// ============ Preference Field Types ============
export type PreferenceFieldType = "multiSelect" | "singleSelect" | "range";

export interface PreferenceFieldConfig {
  key: string;
  label: string;
  type: PreferenceFieldType;
  options?: readonly string[] | readonly { label: string; value: number }[];
  minValue?: number;
  maxValue?: number;
}

// ============ Field Configurations ============
export const PREFERENCE_FIELDS: Record<string, PreferenceFieldConfig> = {
  genderPreference: {
    key: "genderPreference",
    label: "I'm interested in",
    type: "multiSelect",
    options: GENDER_OPTIONS,
  },
  ethnicityPreference: {
    key: "ethnicityPreference",
    label: "Ethnicity Preference",
    type: "multiSelect",
    options: ETHNICITY_OPTIONS,
  },
  religionPreference: {
    key: "religionPreference",
    label: "Religion Preference",
    type: "multiSelect",
    options: RELIGION_OPTIONS,
  },
  distanceMax: {
    key: "distanceMax",
    label: "Maximum Distance",
    type: "singleSelect",
    options: DISTANCE_OPTIONS,
  },
  ageRange: {
    key: "ageRange",
    label: "Age Range",
    type: "range",
    minValue: 18,
    maxValue: 100,
  },
};
