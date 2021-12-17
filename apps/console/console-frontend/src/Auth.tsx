import * as React from "react";
import { Navigate, useLocation } from "react-router-dom";
import { AuthContext } from "./context";
import makeStyles from "@mui/styles/makeStyles";
import createStyles from "@mui/styles/createStyles";
import CircularProgress from "@mui/material/CircularProgress";

export interface AuthProps {
    children: React.ReactNode;
    redirectByReject: string;
}

const useStyles = makeStyles(() =>
    createStyles({
        loader: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
        progress: {
            position: "absolute",
            top: "50%",
            left: "50%",
            marginTop: -12,
            marginLeft: -12,
        },
    })
);

export const Auth = ({ children, redirectByReject }: AuthProps) => {
    const classes = useStyles();
    const location = useLocation();
    const { currentUser, isProgress } = React.useContext(AuthContext);

    if (isProgress) {
        return (
            <div className={classes.loader}>
                <CircularProgress size={68} className={classes.progress} />
            </div>
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
