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

export interface CreateUser {
  email: string,
  phone: string
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
  email?: string;
  phone?: string;
  ageRange?: { min: number; max: number };
  distanceMax?: number;
  genderPreference?: string[];
  ethnicityPreference?: string[];
  religionPreference?: string[];
}

// ============ API Functions ============

// Auth
export async function phoneLogin(phone: string): Promise<LoginResponse> {
  return apiRequest<LoginResponse>('/api/v1/auth/phone/login', {
    method: 'POST',
    body: { phone },
    requiresAuth: false,
  });
}

export async function phoneVerify(verificationId: string, code: string): Promise<AuthResponse> {
  return apiRequest<AuthResponse>('/api/v1/auth/phone/verify', {
    method: 'POST',
    body: { verification_id: verificationId, code },
    requiresAuth: false,
  });
}

// Profile
export async function getMyProfile(): Promise<UserProfile> {
  return apiRequest<UserProfile>('/api/v1/profile/me');
}

export async function updateProfile(data: Partial<ProfileDetails>): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/profile', {
    method: 'POST',
    body: data,
  });
}

// File Upload/Download (R2 Signed URLs)
export interface UploadUrlRequest {
  filename: string;
  content_type: string;
}

export interface SignedUrlResponse {
  upload_url: string;
  key: string;
}

export interface DownloadUrlRequest {
  key: string;
}

export interface DownloadResponse {
  download_url: string;
}

export async function getUploadUrl(filename: string, contentType: string): Promise<SignedUrlResponse> {
  return apiRequest<SignedUrlResponse>('/api/v1/files/upload-url', {
    method: 'POST',
    body: { filename, content_type: contentType },
  });
}

export async function getDownloadUrl(key: string): Promise<DownloadResponse> {
  return apiRequest<DownloadResponse>('/api/v1/files/download-url', {
    method: 'POST',
    body: { key },
  });
}

export interface ViewImagesRequest {
  user_id?: string;
}

export interface ViewImagesResponse {
  images: string[];
}

export async function viewProfileImages(userId?: string): Promise<ViewImagesResponse> {
  return apiRequest<ViewImagesResponse>('/api/v1/files/view', {
    method: 'POST',
    body: userId ? { user_id: userId } : {},
  });
}

// @deprecated - Use getUploadUrl instead
export async function uploadProfileImage(imageUrl: string): Promise<StatusResponse> {
  console.warn('uploadProfileImage is deprecated. Use getUploadUrl instead.');
  return apiRequest<StatusResponse>('/api/v1/profile/images', {
    method: 'POST',
    body: { image_url: imageUrl },
  });
}

export async function finalizeProfile(): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/profile/finalize', {
    method: 'POST',
  });
}

export async function deleteAccount(): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/profile', {
    method: 'DELETE',
  });
}

// Preferences
export async function updatePreferences(prefs: Preferences): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/user/preferences', {
    method: 'POST',
    body: prefs,  // Already in camelCase
  });
}

// User
export async function createUser(user: CreateUser): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/user/create', {
    method: 'POST',
    body: { phone: user.phone, email: user.email },
  });
}

export interface CheckUserExistsRequest {
  phone?: string;
  email?: string;
}

export async function checkUserExists(data: CheckUserExistsRequest): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/user/check', {
    method: 'POST',
    body: { phone: data.phone, email: data.email },
  });
}

// Feed
export async function getFeed(): Promise<FeedResponse> {
  return apiRequest<FeedResponse>('/api/v1/feed');
}

// Interactions
export interface InteractRequest {
  targetUserId: string;
  action: 'LIKE' | 'PASS';
  context?: { type: string; id: string };
  comment?: string;
}

export async function interact(data: InteractRequest): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/interact', {
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
  return apiRequest<UserPrompt[]>('/api/v1/prompts');
}

export async function createPrompt(question: string, answer: string): Promise<StatusResponse> {
  return apiRequest<StatusResponse>('/api/v1/prompts', {
    method: 'POST',
    body: { question, answer },
  });
}

export async function updatePrompt(order: number, question: string, answer: string): Promise<StatusResponse> {
  return apiRequest<StatusResponse>(`/api/v1/prompts/${order}`, {
    method: 'PUT',
    body: { question, answer },
  });
}

export async function deletePrompt(order: number): Promise<StatusResponse> {
  return apiRequest<StatusResponse>(`/api/v1/prompts/${order}`, {
    method: 'DELETE',
  });
}

// Matches
export interface Match {
  id: string;
  user: UserProfile;
  lastMessage?: Message;
  createdAt: string;
}

export interface Message {
  id: string;
  senderId: string;
  text: string;
  createdAt: string;
}

export interface MatchesResponse {
  matches: Match[];
}

export interface MessagesResponse {
  messages: Message[];
}

export async function getMatches(): Promise<MatchesResponse> {
  return apiRequest<MatchesResponse>('/api/v1/matches');
}

export async function getMessages(matchId: string): Promise<MessagesResponse> {
  return apiRequest<MessagesResponse>(`/api/v1/matches/${matchId}/messages`);
}

export async function sendMessage(matchId: string, text: string): Promise<StatusResponse> {
  return apiRequest<StatusResponse>(`/api/v1/matches/${matchId}/messages`, {
    method: 'POST',
    body: { text },
  });
}

// User
export async function getUser(): Promise<UserProfile> {
  return apiRequest<UserProfile>('/api/v1/user/get', {
    method: 'POST',
  });
}
