import * as React from "react";

export type AuthContext = {
    login: boolean;
    signup: boolean;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    currentUser: any;
};

export const authContext = React.createContext({});
