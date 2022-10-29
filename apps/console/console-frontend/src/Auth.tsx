import * as React from "react";
import { Navigate } from "react-router-dom";
import { AuthContext } from "./context";
import CircularProgress from "@mui/material/CircularProgress";
import { styled } from "@mui/material/styles";

export interface AuthProps {
    children: React.ReactNode;
    redirectByReject: string;
}

const StyledDiv = styled("div")(({ theme }) => ({
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
}));

const StyledCircularProgress = styled(CircularProgress)(({ theme }) => ({
    position: "absolute",
    top: "50%",
    left: "50%",
    marginTop: -12,
    marginLeft: -12,
}));

export const Auth = ({ children, redirectByReject }: AuthProps) => {
    const { currentUser, isProgress } = React.useContext(AuthContext);

    if (isProgress) {
        return (
            <StyledDiv>
                <StyledCircularProgress size={68} />
            </StyledDiv>
        );
    } else {
        if (currentUser) {
            console.debug("auth ok");
            return (
                <>
                    {/* <Navigate to={location.pathname} replace /> */}
                    {children}
                </>
            );
        } else {
            console.debug("auth ng");
            return <Navigate to={redirectByReject} replace />;
        }
    }
};
