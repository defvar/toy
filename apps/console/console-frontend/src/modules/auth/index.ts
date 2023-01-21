export type AuthUser = {
    email: string;
    name: string;
};

export const getIdToken = () => {
    if (process.env.TOY_AUTHORIZATION == "none") {
        return Promise.resolve("");
    } else {
        throw new Error("");
    }
};

export const onAuthStateChanged = (fn: (user: AuthUser) => void): void => {
    if (process.env.TOY_AUTHORIZATION == "none") {
        fn({
            email: process.env.DEV_AUTH_USER_EMAIL,
            name: process.env.DEV_AUTH_USER_NAME,
        });
        return;
    }

    throw new Error("");
};

export const signOut = async (): Promise<void> => {
    if (process.env.TOY_AUTHORIZATION == "none") {
        return Promise.resolve();
    }

    throw new Error("");
};
