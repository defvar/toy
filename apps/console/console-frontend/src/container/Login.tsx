import * as React from "react";
import { createStyles, Theme, makeStyles } from "@material-ui/core/styles";
import Container from "@material-ui/core/Container";
import Paper from "@material-ui/core/Paper";
import TextField from "@material-ui/core/TextField";
import Button from "@material-ui/core/Button";
import { AuthContext } from "../context";
import { useHistory } from "react-router-dom";

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
    const { login } = React.useContext(AuthContext);
    const classes = useStyles();
    const history = useHistory();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        login();
        // sueccess ?
        history.push(redirectTo);
    };

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
            </Paper>
        </Container>
    );
};

export default Login;
