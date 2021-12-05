import * as React from "react";
import { HashRouter as Router, Route, Switch } from "react-router-dom";
import AppDrawer from "./container/AppDrawer";
import { Hello } from "./components/Hello";
import { GraphEdit } from "./container/GraphEdit";
import { Graphs } from "./container/Graphs";
import { Login } from "./container/Login";
import { CssBaseline, createTheme, adaptV4Theme } from "@mui/material";
import { ThemeProvider } from "@mui/styles";
import { Theme, StyledEngineProvider } from "@mui/material/styles";
import { AuthProvider, AppContextProvider } from "./context";
import { Auth } from "./Auth";
import { SignOut } from "./container/SignOut";


declare module '@mui/styles/defaultTheme' {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface DefaultTheme extends Theme {}
}


const theme = createTheme(adaptV4Theme({
    palette: {
        mode: "dark",
        primary: {
            main: "#0097a7",
        },
        secondary: {
            main: "#f50057",
        },
        text: {
            primary: "rgba(255,255,255,1)",
            secondary: "rgba(255,255,255,0.7)",
        },
    },
}));

const App = (): JSX.Element => {
    return (
        <AuthProvider>
            <AppContextProvider>
                <StyledEngineProvider injectFirst>
                    <ThemeProvider theme={theme}>
                        <Router>
                            <CssBaseline />
                            <Auth redirectByReject="/login">
                                <AppDrawer>
                                    <Switch>
                                        <Route path="/" exact>
                                            <Hello
                                                compiler="TypeScript"
                                                framework="React"
                                            />
                                        </Route>
                                        <Route path="/graphs" exact>
                                            <Graphs />
                                        </Route>
                                        <Route path="/graphs/:name/edit" exact>
                                            <GraphEdit />
                                        </Route>
                                        <Route path="/signout" exact>
                                            <SignOut />
                                        </Route>
                                    </Switch>
                                </AppDrawer>
                            </Auth>
                            <Route path="/login" exact>
                                <Login redirectTo="/" />
                            </Route>
                        </Router>
                    </ThemeProvider>
                </StyledEngineProvider>
            </AppContextProvider>
        </AuthProvider>
    );
};

export default App;
