/**
 * Firebase Authentication Functions
 * Pure functions - no React, no state management
 */

import auth, { FirebaseAuthTypes } from '@react-native-firebase/auth';
import { GoogleSignin, isErrorWithCode, statusCodes } from '@react-native-google-signin/google-signin';

// Configure Google Sign-In
GoogleSignin.configure({
    webClientId: '761529171367-2eh2930aibssqdgde798vu7en29libp1.apps.googleusercontent.com',
    offlineAccess: true,
});

// Types
export interface FirebaseUser {
    uid: string;
    email: string | null;
    displayName: string | null;
    photoURL: string | null;
}

export interface GoogleSignInResult {
    token: string;
    user: FirebaseUser;
}

export type AuthError = {
    code: string;
    message: string;
    type: 'cancelled' | 'in_progress' | 'play_services' | 'firebase' | 'unknown';
};

/**
 * Sign in with Google and return Firebase token + user info
 */
export async function googleSignIn(): Promise<GoogleSignInResult> {
    // Check Play Services
    await GoogleSignin.hasPlayServices({
        showPlayServicesUpdateDialog: true,
    });

    // Start Google Sign-In
    const { type, data } = await GoogleSignin.signIn();

    if (type === 'cancelled') {
        throw { code: 'CANCELLED', message: 'User cancelled sign-in', type: 'cancelled' } as AuthError;
    }

    if (type !== 'success' || !data.idToken) {
        throw { code: 'NO_TOKEN', message: 'Failed to get ID token', type: 'unknown' } as AuthError;
    }

    // Create Firebase credential
    const googleCredential = auth.GoogleAuthProvider.credential(data.idToken);

    // Sign into Firebase
    const userCredential = await auth().signInWithCredential(googleCredential);

    // Get Firebase ID token
    const firebaseToken = await userCredential.user.getIdToken();

    return {
        token: firebaseToken,
        user: {
            uid: userCredential.user.uid,
            email: userCredential.user.email,
            displayName: userCredential.user.displayName,
            photoURL: userCredential.user.photoURL,
        },
    };
}

/**
 * Send OTP to phone number
 */
export async function sendPhoneOtp(phoneNumber: string): Promise<FirebaseAuthTypes.ConfirmationResult> {
    const formattedNumber = phoneNumber.startsWith('+91') ? phoneNumber : `+91${phoneNumber}`;
    return await auth().signInWithPhoneNumber(formattedNumber);
}

/**
 * Verify OTP and return user credential
 */
export async function verifyPhoneOtp(
    confirmation: FirebaseAuthTypes.ConfirmationResult,
    code: string
): Promise<{ token: string; user: FirebaseUser }> {
    const userCredential = await confirmation.confirm(code);

    if (!userCredential) {
        throw { code: 'NO_CREDENTIAL', message: 'Failed to verify OTP', type: 'unknown' } as AuthError;
    }

    const firebaseToken = await userCredential.user.getIdToken();

    return {
        token: firebaseToken,
        user: {
            uid: userCredential.user.uid,
            email: userCredential.user.email,
            displayName: userCredential.user.displayName,
            photoURL: userCredential.user.photoURL,
        },
    };
}

/**
 * Sign out from Firebase and Google
 */
export async function signOut(): Promise<void> {
    await auth().signOut();
    try {
        await GoogleSignin.signOut();
    } catch {
        // Google sign-out may fail if user didn't sign in with Google
    }
}

/**
 * Get current Firebase user (if any)
 */
export function getCurrentUser(): FirebaseAuthTypes.User | null {
    return auth().currentUser;
}

/**
 * Subscribe to auth state changes
 */
export function onAuthStateChange(
    callback: (user: FirebaseAuthTypes.User | null) => void
): () => void {
    return auth().onAuthStateChanged(callback);
}

/**
 * Parse auth error into user-friendly format
 */
export function parseAuthError(error: any): AuthError {
    if (isErrorWithCode(error)) {
        switch (error.code) {
            case statusCodes.SIGN_IN_CANCELLED:
                return { code: error.code, message: 'Sign-in was cancelled', type: 'cancelled' };
            case statusCodes.IN_PROGRESS:
                return { code: error.code, message: 'Sign-in already in progress', type: 'in_progress' };
            case statusCodes.PLAY_SERVICES_NOT_AVAILABLE:
                return { code: error.code, message: 'Play Services not available', type: 'play_services' };
            default:
                return { code: error.code, message: error.message || 'Sign-in failed', type: 'unknown' };
        }
    }

    return {
        code: error?.code || 'UNKNOWN',
        message: error?.message || 'An unknown error occurred',
        type: error?.code?.startsWith('auth/') ? 'firebase' : 'unknown',
    };
}
