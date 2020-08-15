import * as React from "react";
import { Redirect } from "react-router-dom";
import { AuthContext } from "./context";
import { makeStyles, createStyles } from "@material-ui/core/styles";
import CircularProgress from "@material-ui/core/CircularProgress";

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
    const { currentUser, isProgress } = React.useContext(AuthContext);

    if (isProgress) {
        return (
            <div className={classes.loader}>
                <CircularProgress size={68} className={classes.progress} />
            </div>
        );
    } else {
        if (currentUser) {
            return (
                <>
                    <Redirect to={"/"} />
                    {children}
                </>
            );
        } else {
            return <Redirect to={redirectByReject} />;
        }
    }
};
