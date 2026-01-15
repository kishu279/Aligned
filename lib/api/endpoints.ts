import { apiRequest } from './client';

// ============ Types ============

// Auth
export interface LoginResponse {
  message: string;
  verification_id: string;
}

export interface AuthResponse {
  token: string;
  user: {
    id: string;
    is_profile_complete: boolean;
    is_new_user: boolean;
  };
}

// Profile
export interface UserImage {
  id: string;
  url: string;
  order: number;
}

export interface UserPrompt {
  id: string;
  question: string;
  answer: string;
  order: number;
}

export interface ProfileDetails {
  name?: string;
  bio?: string;
  birthdate?: string;
  pronouns?: string;
  gender?: string;
  sexuality?: string;
  height?: number;
  location?: string;
  job?: string;
  company?: string;
  school?: string;
  ethnicity?: string;
  politics?: string;
  religion?: string;
  relationship_type?: string;
  dating_intention?: string;
  drinks?: string;
  smokes?: string;
}

export interface UserProfile {
  id: string;
  images?: UserImage[];
  prompts?: UserPrompt[];
  details?: ProfileDetails;
}

export interface FeedResponse {
  profiles: UserProfile[];
}

export interface StatusResponse {
  status: string;
  message?: string;
}

// Preferences
export interface Preferences {
  ageRange?: { min: number; max: number };
  distanceMax?: number;
  genderPreference?: string[];
  ethnicityPreference?: string[];
  religionPreference?: string[];
}

// ============ API Functions ============

// Auth
export async function phoneLogin(phone: string): Promise<LoginResponse> {
  return apiRequest<LoginResponse>('/auth/phone/login', {
    method: 'POST',
    body: { phone },
    requiresAuth: false,
  });
}

export async function phoneVerify(verificationId: string, code: string): Promise<AuthResponse> {
  return apiRequest<AuthResponse>('/auth/phone/verify', {
    method: 'POST',
    body: { verification_id: verificationId, code },
    requiresAuth: false,
  });
}

// Profile
export async function getMyProfile(): Promise<UserProfile> {
  return apiRequest<UserProfile>('/profile/me');
}

export async function updateProfile(data: Partial<ProfileDetails>): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/profile', {
    method: 'POST',
    body: data,
  });
}

export async function uploadProfileImage(imageUrl: string): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/profile/images', {
    method: 'POST',
    body: { image_url: imageUrl },
  });
}

export async function deleteAccount(): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/profile', {
    method: 'DELETE',
  });
}

// Preferences
export async function updatePreferences(prefs: Preferences): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/user/preferences', {
    method: 'POST',
    body: {
      age_range: prefs.ageRange,
      distance_max: prefs.distanceMax,
      gender_preference: prefs.genderPreference,
      ethnicity_preference: prefs.ethnicityPreference,
      religion_preference: prefs.religionPreference,
    },
  });
}

// Feed
export async function getFeed(): Promise<FeedResponse> {
  return apiRequest<FeedResponse>('/feed');
}

// Interactions
export interface InteractRequest {
  targetUserId: string;
  action: 'LIKE' | 'PASS';
  context?: { type: string; id: string };
  comment?: string;
}

export async function interact(data: InteractRequest): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/interact', {
    method: 'POST',
    body: {
      target_user_id: data.targetUserId,
      action: data.action,
      context: data.context,
      comment: data.comment,
    },
  });
}

// Prompts
export async function getPrompts(): Promise<UserPrompt[]> {
  return apiRequest<UserPrompt[]>('/prompts');
}

export async function createPrompt(question: string, answer: string): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/prompts', {
    method: 'POST',
    body: { question, answer },
  });
}
