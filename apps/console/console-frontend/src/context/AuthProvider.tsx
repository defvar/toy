import * as React from "react";

export type AuthContextData = {
    login: () => void;
    signup: () => void;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    currentUser: any;
};

export const AuthContext = React.createContext({
    login: () => {
        return;
    },
    signup: () => {
        return;
    },
    currentUser: {},
} as AuthContextData);

export interface AuthProviderProps {
    children: React.ReactNode;
}

export const AuthProvider = ({ children }: AuthProviderProps) => {
    const [currentUser, setCurrentUser] = React.useState(null);

    // React.useEffect(() => {
    //     onAuthStateChange...?
    // }, []);

    const login = () => {
        setCurrentUser({ user: "dev" });
        return;
    };

    const signup = () => {
        return;
    };

    return (
        <AuthContext.Provider value={{ login, signup, currentUser }}>
            {children}
        </AuthContext.Provider>
    );
};
