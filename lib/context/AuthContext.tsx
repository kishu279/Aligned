import React, { createContext, ReactNode, useContext, useEffect, useState } from 'react';
import { clearToken, getToken, setToken } from '../api/client';
import { AuthResponse, getMyProfile, phoneLogin, phoneVerify, UserProfile } from '../api/endpoints';

interface AuthContextType {
    isLoading: boolean;
    isAuthenticated: boolean;
    user: AuthResponse['user'] | null;
    profile: UserProfile | null;

    // Auth actions
    login: (phone: string) => Promise<string>; // returns verification_id
    verify: (verificationId: string, code: string) => Promise<AuthResponse>;
    logout: () => Promise<void>;
    refreshProfile: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [isLoading, setIsLoading] = useState(true);
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [user, setUser] = useState<AuthResponse['user'] | null>(null);
    const [profile, setProfile] = useState<UserProfile | null>(null);

    // Check for existing token on mount
    useEffect(() => {
        checkAuth();
    }, []);

    const checkAuth = async () => {
        try {
            const token = await getToken();
            if (token) {
                // Token exists, try to get profile
                const profileData = await getMyProfile();
                setProfile(profileData);
                setIsAuthenticated(true);
            }
        } catch (error) {
            // Token invalid or expired
            await clearToken();
        } finally {
            setIsLoading(false);
        }
    };

    const login = async (phone: string): Promise<string> => {
        const response = await phoneLogin(phone);
        return response.verification_id;
    };

    const verify = async (verificationId: string, code: string): Promise<AuthResponse> => {
        const response = await phoneVerify(verificationId, code);
        await setToken(response.token);
        setUser(response.user);
        setIsAuthenticated(true);

        // Fetch profile after login
        try {
            const profileData = await getMyProfile();
            setProfile(profileData);
        } catch {
            // Profile may not exist yet for new users
        }

        return response;
    };

    const logout = async () => {
        await clearToken();
        setUser(null);
        setProfile(null);
        setIsAuthenticated(false);
    };

    const refreshProfile = async () => {
        try {
            const profileData = await getMyProfile();
            setProfile(profileData);
        } catch (error) {
            console.error('Failed to refresh profile:', error);
        }
    };

    return (
        <AuthContext.Provider
            value={{
                isLoading,
                isAuthenticated,
                user,
                profile,
                login,
                verify,
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
