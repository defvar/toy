import * as React from "react";

export type AppContextData = {};

export const AppContext = React.createContext({} as AppContextData);

export interface AppContextProviderProps {
    children: React.ReactNode;
}

export const AppContextProvider = ({ children }: AppContextProviderProps) => {
    const [currentCtx] = React.useState(null);

    return (
        <AppContext.Provider value={currentCtx}>{children}</AppContext.Provider>
    );
};
