import * as firebase from "firebase/app";
import { GoogleAuthProvider, getAuth, signInWithPopup as googlePopup } from "firebase/auth";
import { AuthUser } from "../auth";

export const app = firebase.initializeApp({
    apiKey: process.env.FIREBASE_KEY,
    authDomain: process.env.FIREBASE_DOMAIN,
    databaseURL: process.env.FIREBASE_DATABASE,
    projectId: process.env.FIREBASE_PROJECT_ID,
    storageBucket: process.env.FIREBASE_STORAGE_BUCKET,
    messagingSenderId: process.env.FIREBASE_SENDER_ID,
});

export const googleAuthProvider = () => {
    return new GoogleAuthProvider();
};

export const auth = () => {
    return getAuth();
};

export const signInWithPopup = async (): Promise<AuthUser> => {
    const r = await googlePopup(getAuth(), googleAuthProvider());
    return {
        email: r.user.email,
        name: r.user.displayName,
    };
}
