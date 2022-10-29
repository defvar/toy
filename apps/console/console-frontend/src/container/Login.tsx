import * as React from "react";
import { Theme } from "@mui/material/styles";
import Container from "@mui/material/Container";
import Paper from "@mui/material/Paper";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import CircularProgress from "@mui/material/CircularProgress";
import { AuthContext } from "../context";
import { Navigate, useNavigate } from "react-router-dom";
import { styled } from "@mui/material/styles";

const FormRow = styled("div")(({ theme }) => ({
    margin: theme.spacing(5),
    textAlign: "center",
}));

const FormControl = styled(TextField)(({ theme }) => ({
    width: "25em",
}));

const FormButton = styled(Button)(({ theme }) => ({
    width: "25em",
}));

export interface LoginProps {
    redirectTo: string;
}

export const Login = ({ redirectTo }: LoginProps) => {
    const { login, signinWithGoogle, isProgress, currentUser } =
        React.useContext(AuthContext);
    const navigate = useNavigate();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        login();
        // sueccess ?
        navigate(redirectTo);
    };

    const handleSigninWithGoogle = () => {
        signinWithGoogle();
        navigate(redirectTo);
    };

    if (isProgress) {
        return <CircularProgress size={68} />;
    } else {
        if (!isProgress && currentUser) {
            return <Navigate to={redirectTo} />;
        }
    }

    return (
        <Container
            fixed
            sx={{
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                height: "100vh",
            }}
        >
            <Paper elevation={3}>
                <form onSubmit={handleSubmit} noValidate autoComplete="off">
                    <FormRow>
                        <FormControl id="user" label="UserName" />
                    </FormRow>
                    <FormRow>
                        <FormControl
                            id="password"
                            label="Password"
                            type="password"
                            autoComplete="current-password"
                        />
                    </FormRow>
                    <FormRow>
                        <FormButton
                            type="submit"
                            variant="contained"
                            size="large"
                            color="primary"
                        >
                            Login
                        </FormButton>
                    </FormRow>
                </form>
                <FormRow>
                    <FormButton
                        onClick={handleSigninWithGoogle}
                        variant="contained"
                        size="large"
                        color="primary"
                    >
                        Sign in with Google
                    </FormButton>
                </FormRow>
            </Paper>
        </Container>
    );
};

export default Login;
