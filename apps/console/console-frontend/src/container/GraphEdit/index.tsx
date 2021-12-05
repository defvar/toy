import * as React from "react";
import { useParams } from "react-router-dom";
import { Theme, useTheme } from "@mui/material/styles";
import createStyles from '@mui/styles/createStyles';
import makeStyles from '@mui/styles/makeStyles';
import Typography from "@mui/material/Typography";
import RefreshIcon from "@mui/icons-material/Refresh";
import IconButton from "@mui/material/IconButton";
import { Sidebar, Chart } from "./chart";
import ZoomInIcon from "@mui/icons-material/ZoomIn";
import ZoomOutIcon from "@mui/icons-material/ZoomOut";
import CircularProgress from "@mui/material/CircularProgress";
import { Resource } from "../../modules/common";
import {
    GraphEditState,
    Actions,
    reducer,
    initialState,
} from "../../modules/graphEdit";
import { Resizable } from "react-resizable";
import AppBar from "@mui/material/AppBar";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import {
    ServiceResponse,
    fetchServices,
    fetchGraph,
    GraphResponse,
} from "../../modules/api/toy-api";
import { NodeEditor } from "./NodeEditor";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
            display: "flex",
        },
        heading: {
            fontSize: theme.typography.pxToRem(15),
        },
        leftPane: {
            position: "relative",
        },
        rightPane: {
            marginLeft: theme.spacing(3),
            flexGrow: 1,
            backgroundColor: theme.palette.background.paper,
            zIndex: 990,
        },
        chartCanvas: {
            overflow: "hidden",
            maxHeight: "80vh",
        },
        loader: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
        progress: {},
        resizeHandle: {
            position: "absolute",
            width: "4px",
            height: "100px",
            backgroundColor: "#1153aa",
            opacity: "0.75",
            top: "30%",
            marginTop: "-4px",
            right: "-11px",
            cursor: "ew-resize",
        },
    })
);

interface GraphEditSuspenseProps {
    state: GraphEditState;
    dispatch: React.Dispatch<Actions>;
    serviceResource: Resource<ServiceResponse>;
    graphResource: Resource<GraphResponse>;
}

const SidebarSuspense = (props: GraphEditSuspenseProps) => {
    const r = props.serviceResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetServices",
            payload: r,
        });
    }, [r]);
    return (
        <Sidebar
            services={props.state.services}
            namespaces={props.state.namespaces}
        />
    );
};

const ChartSuspense = (props: GraphEditSuspenseProps) => {
    const graph = props.graphResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetGraph",
            payload: graph,
        });
    }, [graph]);

    return <Chart data={props.state.graph} dispatch={props.dispatch} />;
};

export const GraphEdit = () => {
    const { name } = useParams();
    const classes = useStyles();
    const theme = useTheme();

    const [serviceResource, setServiceResource] = React.useState(() =>
        fetchServices()
    );
    const [graphResource, setGraphResource] = React.useState(() =>
        fetchGraph(name)
    );
    const [state, dispatch] = React.useReducer(reducer, initialState);
    const [tabNumber, setTabNumber] = React.useState(0);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const onSidebarRefleshClick = React.useCallback(() => {
        setServiceResource(() => fetchServices());
    }, []);

    const handleZoomIn = React.useCallback(() => {
        dispatch({
            type: "ZoomChart",
            payload: 0.1,
        });
    }, []);

    const handleZoomOut = React.useCallback(() => {
        dispatch({
            type: "ZoomChart",
            payload: -0.1,
        });
    }, []);

    const [size, setSize] = React.useState(() => {
        return { width: theme.breakpoints.values.md };
    });

    const onResize = (_event, { size }) => {
        setSize({ width: size.width });
    };

    const handleTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: number) => {
            setTabNumber(newValue);
        },
        [state.services]
    );

    return (
        <div className={classes.root}>
            <Resizable
                width={size.width}
                height={Infinity}
                className={classes.leftPane}
                onResize={onResize}
                handle={<span className={classes.resizeHandle} />}
                resizeHandles={["e"]}
            >
                <div
                    style={{
                        width: size.width + "px",
                    }}
                >
                    <Typography className={classes.heading}>{name}</Typography>
                    <IconButton aria-label="refresh" onClick={onChartRefleshClick} size="large">
                        <RefreshIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-in" onClick={handleZoomIn} size="large">
                        <ZoomInIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-out" onClick={handleZoomOut} size="large">
                        <ZoomOutIcon />
                    </IconButton>
                    <div
                        style={{
                            width: size.width + "px",
                        }}
                        className={classes.chartCanvas}
                    >
                        <React.Suspense
                            fallback={
                                <div className={classes.loader}>
                                    <CircularProgress
                                        size={68}
                                        className={classes.progress}
                                    />
                                </div>
                            }
                        >
                            <ChartSuspense
                                state={state}
                                dispatch={dispatch}
                                serviceResource={serviceResource}
                                graphResource={graphResource}
                            />
                        </React.Suspense>
                    </div>
                </div>
            </Resizable>
            <div className={classes.rightPane}>
                <AppBar position={"relative"}>
                    <Tabs
                        value={tabNumber}
                        onChange={handleTabChange}
                        aria-label="tabs"
                    >
                        <Tab label="Services" />
                    </Tabs>
                </AppBar>
                <div hidden={tabNumber !== 0}>
                    <IconButton aria-label="refresh" onClick={onSidebarRefleshClick} size="large">
                        <RefreshIcon />
                    </IconButton>
                    <React.Suspense
                        fallback={
                            <div className={classes.loader}>
                                <CircularProgress
                                    size={68}
                                    className={classes.progress}
                                />
                            </div>
                        }
                    >
                        <SidebarSuspense
                            state={state}
                            dispatch={dispatch}
                            serviceResource={serviceResource}
                            graphResource={graphResource}
                        />
                    </React.Suspense>
                </div>
            </div>
            <NodeEditor
                state={state}
                dispatch={dispatch}
                open={!!state.edit.id}
            />
        </div>
    );
};

export default GraphEdit;
