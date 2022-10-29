import * as React from "react";
import { HashRouter as Router, Route, Routes, Outlet } from "react-router-dom";
import AppDrawer from "./container/AppDrawer";
import { Hello } from "./components/Hello";
import { GraphEdit } from "./container/GraphEdit";
import { Graphs } from "./container/Graphs";
import { Login } from "./container/Login";
import { ManageAuth } from "./container/ManageAuth";
import { CssBaseline } from "@mui/material";
import {
    StyledEngineProvider,
    createTheme,
    ThemeProvider,
} from "@mui/material/styles";
import { AuthProvider, AppContextProvider } from "./context";
import { Auth } from "./Auth";
import { SignOut } from "./container/SignOut";

const theme = createTheme({
    palette: {
        mode: "dark",
    },
});

const App = (): JSX.Element => {
    return (
        <AuthProvider>
            <AppContextProvider>
                <StyledEngineProvider injectFirst>
                    <ThemeProvider theme={theme}>
                        <CssBaseline />
                        <Router>
                            <Auth redirectByReject="/login">
                                <AppDrawer>
                                    <Routes>
                                        <Route
                                            path="/"
                                            element={
                                                <Hello
                                                    compiler="TypeScript"
                                                    framework="React"
                                                />
                                            }
                                        />
                                        <Route
                                            path="/graphs"
                                            element={<Graphs />}
                                        />
                                        <Route
                                            path="/graphs/:name/edit"
                                            element={<GraphEdit />}
                                        />
                                        <Route
                                            path="/manageAuth"
                                            element={<ManageAuth />}
                                        />
                                        <Route
                                            path="/signout"
                                            element={<SignOut />}
                                        />
                                        <Route
                                            path="/login"
                                            element={<Login redirectTo="/" />}
                                        />
                                        <Route
                                            index
                                            element={
                                                <div>
                                                    <Hello
                                                        compiler="TypeScript"
                                                        framework="React"
                                                    />
                                                    <Outlet />
                                                </div>
                                            }
                                        />
                                    </Routes>
                                </AppDrawer>
                            </Auth>
                        </Router>
                    </ThemeProvider>
                </StyledEngineProvider>
            </AppContextProvider>
        </AuthProvider>
    );
};

export default App;
