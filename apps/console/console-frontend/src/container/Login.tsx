import * as React from "react";
import { createStyles, Theme, makeStyles } from "@material-ui/core/styles";
import Container from "@material-ui/core/Container";
import Paper from "@material-ui/core/Paper";
import TextField from "@material-ui/core/TextField";
import Button from "@material-ui/core/Button";
import CircularProgress from "@material-ui/core/CircularProgress";
import { AuthContext } from "../context";
import { useHistory } from "react-router-dom";
import { Redirect } from "react-router-dom";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            height: "100vh",
        },
        formRow: {
            margin: theme.spacing(5),
            textAlign: "center",
        },
        formControl: {
            width: "25em",
        },
        formButton: {
            width: "20em",
        },
    })
);

export interface LoginProps {
    redirectTo: string;
}

export const Login = ({ redirectTo }: LoginProps) => {
    const {
        login,
        signinWithGoogle,
        isProgress,
        currentUser,
    } = React.useContext(AuthContext);
    const classes = useStyles();
    const history = useHistory();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        login();
        // sueccess ?
        history.push(redirectTo);
    };

    const handleSigninWithGoogle = () => {
        signinWithGoogle();
        history.push(redirectTo);
    };

    if (isProgress) {
        return <CircularProgress size={68} />;
    } else {
        if (!isProgress && currentUser) {
            return <Redirect to={redirectTo} />;
        }
    }

    return (
        <Container fixed className={classes.root}>
            <Paper elevation={3}>
                <form onSubmit={handleSubmit} noValidate autoComplete="off">
                    <div className={classes.formRow}>
                        <TextField
                            id="user"
                            className={classes.formControl}
                            label="UserName"
                        />
                    </div>
                    <div className={classes.formRow}>
                        <TextField
                            id="password"
                            className={classes.formControl}
                            label="Password"
                            type="password"
                            autoComplete="current-password"
                        />
                    </div>
                    <div className={classes.formRow}>
                        <Button
                            type="submit"
                            className={classes.formButton}
                            variant="contained"
                            size="large"
                            color="primary"
                        >
                            Login
                        </Button>
                    </div>
                </form>
                <div className={classes.formRow}>
                    <Button
                        className={classes.formButton}
                        onClick={handleSigninWithGoogle}
                        variant="contained"
                        size="large"
                        color="primary"
                    >
                        Sign in with Google
                    </Button>
                </div>
            </Paper>
        </Container>
    );
};

export default Login;
