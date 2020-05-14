import * as React from "react";
import { Redirect } from "react-router-dom";
import { AuthContext } from "./context";

export interface AuthProps {
    children: React.ReactNode;
    redirectByReject: string;
}

export const Auth = ({ children, redirectByReject }: AuthProps) => {
    const { currentUser } = React.useContext(AuthContext);
    const e: React.ReactNode = currentUser ? (
        children
    ) : (
        <Redirect to={redirectByReject} />
    );
    return <>{e}</>;
};
