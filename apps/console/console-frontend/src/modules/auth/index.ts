import { auth, signInWithPopup } from "../firebase";

export type AuthUser = {
    email: string;
    name: string;
};

export const getIdToken = () => {
    if (process.env.DEV_AUTH == "none") {
        return Promise.resolve("");
    } else {
        return auth().currentUser.getIdToken();
    }
};

export const onAuthStateChanged = (fn: (user: AuthUser) => void): void => {
    if (process.env.DEV_AUTH == "none") {
        fn({
            email: process.env.DEV_AUTH_USER_EMAIL,
            name: process.env.DEV_AUTH_USER_NAME,
        });
        return;
    }

    auth().onAuthStateChanged((fireBaseUser) => {
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
    if (process.env.DEV_AUTH == "none") {
        return Promise.resolve({
            email: process.env.DEV_AUTH_USER_EMAIL,
            name: process.env.DEV_AUTH_USER_NAME,
        });
    }

    const r = await signInWithPopup();
    return r;
};

export const signOut = async (): Promise<void> => {
    if (process.env.DEV_AUTH == "none") {
        return Promise.resolve();
    }

    return auth().signOut();
};
