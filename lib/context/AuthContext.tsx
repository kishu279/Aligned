import React, { createContext, ReactNode, useContext, useEffect, useState } from 'react';
import { clearToken, setToken } from '../api/client';
import { getMyProfile, UserProfile } from '../api/endpoints';
import { Alert } from 'react-native';
import { router } from 'expo-router';

// Import Firebase auth functions
import {
    FirebaseUser,
    googleSignIn,
    sendPhoneOtp,
    verifyPhoneOtp,
    signOut as firebaseSignOut,
    onAuthStateChange,
    parseAuthError,
} from '../auth/firebase-auth';
import { FirebaseAuthTypes } from '@react-native-firebase/auth';

// Re-export FirebaseUser type
export type { FirebaseUser } from '../auth/firebase-auth';

interface AuthContextType {
    isLoading: boolean;
    isAuthenticated: boolean;
    firebaseUser: FirebaseUser | null;
    token: string | null;
    profile: UserProfile | null;

    // Auth actions
    handleGoogleSignIn: () => Promise<void>;
    sendOtp: (phone: string) => Promise<FirebaseAuthTypes.ConfirmationResult>;
    verifyOtp: (confirmation: FirebaseAuthTypes.ConfirmationResult, code: string) => Promise<void>;
    logout: () => Promise<void>;
    refreshProfile: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [isLoading, setIsLoading] = useState(true);
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [firebaseUser, setFirebaseUser] = useState<FirebaseUser | null>(null);
    const [token, setTokenState] = useState<string | null>(null);
    const [profile, setProfile] = useState<UserProfile | null>(null);

    // Listen for Firebase auth state changes on mount
    useEffect(() => {
        console.log('[AuthContext] Setting up auth state listener...');

        const unsubscribe = onAuthStateChange(async (user) => {
            console.log('[AuthContext] Auth state changed:', user ? 'User logged in' : 'No user');

            if (user) {
                // User is signed in
                const idToken = await user.getIdToken();
                await setToken(idToken);
                setTokenState(idToken);
                setFirebaseUser({
                    uid: user.uid,
                    email: user.email,
                    displayName: user.displayName,
                    photoURL: user.photoURL,
                });
                setIsAuthenticated(true);
            } else {
                // User is signed out
                await clearToken();
                setTokenState(null);
                setFirebaseUser(null);
                setProfile(null);
                setIsAuthenticated(false);
            }

            setIsLoading(false);
        });

        // Cleanup subscription
        return unsubscribe;
    }, []);

    // Google Sign-In handler
    const handleGoogleSignIn = async (): Promise<void> => {
        console.log('[AuthContext] handleGoogleSignIn called');

        if (isAuthenticated) {
            console.log('[AuthContext] Already authenticated, redirecting...');
            router.replace('/(tabs)');
            return;
        }

        try {
            const result = await googleSignIn();
            console.log('[AuthContext] Google sign-in successful');

            // Save token
            await setToken(result.token);
            setTokenState(result.token);
            setFirebaseUser(result.user);
            setIsAuthenticated(true);

            router.replace('/(tabs)');
        } catch (error: any) {
            console.error('[AuthContext] Google sign-in error:', error);

            const authError = parseAuthError(error);

            if (authError.type === 'cancelled') {
                // User cancelled, do nothing
                return;
            }

            Alert.alert('Sign In Error', authError.message);
        }
    };

    // Phone OTP handlers
    const sendOtp = async (phone: string): Promise<FirebaseAuthTypes.ConfirmationResult> => {
        console.log('[AuthContext] Sending OTP to:', phone);
        return await sendPhoneOtp(phone);
    };

    const verifyOtp = async (
        confirmation: FirebaseAuthTypes.ConfirmationResult,
        code: string
    ): Promise<void> => {
        console.log('[AuthContext] Verifying OTP...');

        try {
            const result = await verifyPhoneOtp(confirmation, code);

            // Save token
            await setToken(result.token);
            setTokenState(result.token);
            setFirebaseUser(result.user);
            setIsAuthenticated(true);

            console.log('[AuthContext] OTP verified successfully');
        } catch (error: any) {
            console.error('[AuthContext] OTP verification error:', error);
            throw error;
        }
    };

    // Logout handler
    const logout = async (): Promise<void> => {
        console.log('[AuthContext] Logging out...');
        await firebaseSignOut();
        await clearToken();
        setFirebaseUser(null);
        setTokenState(null);
        setProfile(null);
        setIsAuthenticated(false);
    };

    // Refresh profile from backend
    const refreshProfile = async (): Promise<void> => {
        try {
            const profileData = await getMyProfile();
            setProfile(profileData);
        } catch (error) {
            console.error('[AuthContext] Failed to refresh profile:', error);
        }
    };

    return (
        <AuthContext.Provider
            value={{
                isLoading,
                isAuthenticated,
                firebaseUser,
                token,
                profile,
                handleGoogleSignIn,
                sendOtp,
                verifyOtp,
                logout,
                refreshProfile,
            }}
        >
            {children}
        </AuthContext.Provider>
    );
}

export function useAuth() {
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider');
    }
    return context;
}
