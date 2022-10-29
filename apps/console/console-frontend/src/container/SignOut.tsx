import * as React from "react";
import CircularProgress from "@mui/material/CircularProgress";
import * as auth from "../modules/auth";
import { Navigate } from "react-router-dom";

export const SignOut = () => {
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
                <div
                    style={{
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "center",
                    }}
                >
                    <CircularProgress
                        size={68}
                        sx={{
                            position: "absolute",
                            top: "50%",
                            left: "50%",
                            marginTop: -12,
                            marginLeft: -12,
                        }}
                    />
                </div>
            ) : (
                <Navigate to={"/"} />
            )}
        </>
    );
};
