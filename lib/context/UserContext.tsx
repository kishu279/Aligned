import React, { createContext, ReactNode, useContext, useState, useCallback } from 'react';
import {
    getMyProfile,
    getPrompts,
    updateProfile as updateProfileApi,
    updatePreferences as updatePreferencesApi,
    ProfileDetails,
    UserImage,
    UserPrompt,
    Preferences,
    StatusResponse,
} from '../api/endpoints';

// User state interface
interface UserState {
    email: string | null;
    phone: string | null;
    profile: ProfileDetails | null;
    images: UserImage[] | null;
    prompts: UserPrompt[] | null;
    preferences: Preferences | null;
    isLoading: boolean;
    isProfileComplete: boolean;
}

// Context type with all functions
interface UserContextType extends UserState {
    // Identity
    setUserIdentity: (email: string | null, phone: string | null) => void;

    // Fetch functions
    fetchProfile: () => Promise<void>;
    fetchPrompts: () => Promise<void>;
    refreshAll: () => Promise<void>;

    // Update functions
    updateProfileData: (data: Partial<ProfileDetails>) => Promise<StatusResponse>;
    updatePreferencesData: (prefs: Partial<Preferences>) => Promise<StatusResponse>;

    // Clear on logout
    clearUser: () => void;
}

const UserContext = createContext<UserContextType | undefined>(undefined);

export function UserProvider({ children }: { children: ReactNode }) {
    const [email, setEmail] = useState<string | null>(null);
    const [phone, setPhone] = useState<string | null>(null);
    const [profile, setProfile] = useState<ProfileDetails | null>(null);
    const [images, setImages] = useState<UserImage[] | null>(null);
    const [prompts, setPrompts] = useState<UserPrompt[] | null>(null);
    const [preferences, setPreferences] = useState<Preferences | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [isProfileComplete, setIsProfileComplete] = useState(false);

    // Set user identity after auth
    const setUserIdentity = useCallback((userEmail: string | null, userPhone: string | null) => {
        console.log('[UserContext] Setting identity:', { email: userEmail, phone: userPhone });
        setEmail(userEmail);
        setPhone(userPhone);
    }, []);

    // Fetch profile from API
    const fetchProfile = useCallback(async () => {
        try {
            setIsLoading(true);
            console.log('[UserContext] Fetching profile...');
            const response = await getMyProfile();

            if (response.details) {
                setProfile(response.details);
            }
            if (response.images) {
                setImages(response.images);
                setIsProfileComplete(response.images.length >= 6);
            }
            if (response.prompts) {
                setPrompts(response.prompts);
            }
            console.log('[UserContext] Profile fetched:', response);
        } catch (error) {
            console.error('[UserContext] Failed to fetch profile:', error);
        } finally {
            setIsLoading(false);
        }
    }, []);

    // Fetch prompts from API
    const fetchPrompts = useCallback(async () => {
        try {
            console.log('[UserContext] Fetching prompts...');
            const response = await getPrompts();
            setPrompts(response);
            console.log('[UserContext] Prompts fetched:', response);
        } catch (error) {
            console.error('[UserContext] Failed to fetch prompts:', error);
        }
    }, []);

    // Refresh all user data
    const refreshAll = useCallback(async () => {
        console.log('[UserContext] Refreshing all user data...');
        setIsLoading(true);
        try {
            await Promise.all([fetchProfile(), fetchPrompts()]);
        } finally {
            setIsLoading(false);
        }
    }, [fetchProfile, fetchPrompts]);

    // Update profile via API
    const updateProfileData = useCallback(async (data: Partial<ProfileDetails>): Promise<StatusResponse> => {
        console.log('[UserContext] Updating profile:', data);
        const response = await updateProfileApi(data);
        if (response.status === 'success') {
            // Update local state with new data
            setProfile(prev => prev ? { ...prev, ...data } : data as ProfileDetails);
        }
        return response;
    }, []);

    // Update preferences via API
    const updatePreferencesData = useCallback(async (prefs: Partial<Preferences>): Promise<StatusResponse> => {
        // Include email/phone for user identification
        const prefsWithIdentity = {
            ...prefs,
            email: email || undefined,
            phone: phone || undefined,
        };
        console.log('[UserContext] Updating preferences:', prefsWithIdentity);
        const response = await updatePreferencesApi(prefsWithIdentity);
        if (response.status === 'success') {
            setPreferences(prev => prev ? { ...prev, ...prefs } : prefs as Preferences);
        }
        return response;
    }, [email, phone]);

    // Clear all user data on logout
    const clearUser = useCallback(() => {
        console.log('[UserContext] Clearing user data');
        setEmail(null);
        setPhone(null);
        setProfile(null);
        setImages(null);
        setPrompts(null);
        setPreferences(null);
        setIsProfileComplete(false);
    }, []);

    const value: UserContextType = {
        // State
        email,
        phone,
        profile,
        images,
        prompts,
        preferences,
        isLoading,
        isProfileComplete,
        // Functions
        setUserIdentity,
        fetchProfile,
        fetchPrompts,
        refreshAll,
        updateProfileData,
        updatePreferencesData,
        clearUser,
    };

    return <UserContext.Provider value={value}>{children}</UserContext.Provider>;
}

// Hook to use user context
export function useUser() {
    const context = useContext(UserContext);
    if (context === undefined) {
        throw new Error('useUser must be used within a UserProvider');
    }
    return context;
}

// Export types
export type { UserState, UserContextType };
