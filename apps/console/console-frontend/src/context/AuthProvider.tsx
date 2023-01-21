import * as React from "react";
import * as auth from "../modules/auth";

export type AuthContextData = {
    login: () => void;
    signup: () => void;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    currentUser?: auth.AuthUser;
    isProgress: boolean;
};

export const AuthContext = React.createContext({
    login: () => {
        return;
    },
    signup: () => {
        return;
    },
    currentUser: null,
    isProgress: false,
} as AuthContextData);

export interface AuthProviderProps {
    children: React.ReactNode;
}

export const AuthProvider = ({ children }: AuthProviderProps) => {
    const [currentUser, setCurrentUser] = React.useState(null);
    const [isProgress, setProgress] = React.useState(true);

    React.useEffect(() => {
        auth.onAuthStateChanged((user) => {
            if (user) {
                setCurrentUser(user);
            } else {
                setCurrentUser(null);
            }
            setProgress(false);
        });
    }, []);

    const login = () => {
        setCurrentUser({ user: "dev" });
        return;
    };

    const signup = () => {
        return;
    };

    return (
        <AuthContext.Provider
            value={{
                login,
                signup,
                currentUser,
                isProgress,
            }}
        >
            {children}
        </AuthContext.Provider>
    );
};
