import * as React from "react";
import createStyles from '@mui/styles/createStyles';
import makeStyles from '@mui/styles/makeStyles';
import CircularProgress from "@mui/material/CircularProgress";
import * as auth from "../modules/auth";
import { Redirect } from "react-router-dom";

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

export const SignOut = () => {
    const classes = useStyles();
    const [isLoading, setLoading] = React.useState(true);

    React.useEffect(() => {
        let cleanedUp = false;
        auth.signOut().then(() => {
            if (!cleanedUp) {
                setLoading(false);
            }
        });
        const cleanup = () => {
            cleanedUp = true;
        };
        return cleanup;
    }, []);

    return (
        <>
            {isLoading ? (
                <div className={classes.loader}>
                    <CircularProgress size={68} className={classes.progress} />
                </div>
            ) : (
                <Redirect to={"/"} />
            )}
        </>
    );
};
