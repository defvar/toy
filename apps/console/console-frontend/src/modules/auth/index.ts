import { app, googleAuthProvider } from "../firebase";

export type AuthUser = {
    email: string;
    name: string;
};

export const getIdToken = () => {
    return app.auth().currentUser.getIdToken();
};

export const onAuthStateChanged = (fn: (user: AuthUser) => void): void => {
    app.auth().onAuthStateChanged((fireBaseUser) => {
        if (fireBaseUser) {
            fn({
                email: fireBaseUser.email,
                name: fireBaseUser.displayName,
            });
        } else {
            fn(null);
        }
    });
};

export const signinWithPopupToGoogle = async (): Promise<AuthUser> => {
    const r = await app.auth().signInWithPopup(googleAuthProvider());
    return {
        email: r.user.email,
        name: r.user.displayName,
    };
};

export const signOut = async (): Promise<void> => {
    return app.auth().signOut();
};
