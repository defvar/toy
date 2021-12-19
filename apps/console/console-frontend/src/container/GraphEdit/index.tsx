import * as React from "react";
import { useParams } from "react-router-dom";
import { Theme, useTheme, styled } from "@mui/material/styles";
import createStyles from "@mui/styles/createStyles";
import makeStyles from "@mui/styles/makeStyles";
import Typography from "@mui/material/Typography";
import RefreshIcon from "@mui/icons-material/Refresh";
import IconButton from "@mui/material/IconButton";
import Box from "@mui/material/Box";
import Tab from "@mui/material/Tab";
import Tabs from "@mui/material/Tabs";
import { Sidebar, Chart } from "./chart";
import { Resource } from "../../modules/common";
import {
    GraphEditState,
    Actions,
    reducer,
    initialState,
    ServiceState,
} from "../../modules/graphEdit";
import {
    ServiceResponse,
    fetchServices,
    fetchGraph,
    GraphResponse,
} from "../../modules/api/toy-api";
import { NodeEditor } from "./NodeEditor";
import { ChartData } from "../../modules/graphEdit/types";
import CircularProgress from "../../components/progress/CircularProgress";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import { Resizable } from "react-resizable";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        resizeHandle: {
            position: "absolute",
            width: "1px",
            height: "100%",
            backgroundColor: theme.palette.divider,
            opacity: "0.75",
            top: "0",
            cursor: "ew-resize",
        },
        resizeHandleBottom: {
            position: "absolute",
            width: "100%",
            height: "1px",
            backgroundColor: theme.palette.divider,
            opacity: "0.75",
            bottom: "0",
            cursor: "ns-resize",
            zIndex: 11000,
        },
    })
);

interface GraphEditSuspenseProps {
    state: GraphEditState;
    dispatch: React.Dispatch<Actions>;
    serviceResource: Resource<ServiceResponse>;
    graphResource: Resource<GraphResponse>;
    height?: string | number;
}

const _testServices = {
    "a.b.c": {
        fullName: "a.b.c",
        name: "c",
        namespace: "a.b",
        description: "aaaaaa",
        inPort: 1,
        outPort: 1,
        configSchema: null,
        portType: "Flow",
    },
} as { [fullName: string]: ServiceState };

const _testNamespaces = {
    "a.b": ["a.b.c"],
};

const _testChartData = {
    elements: [
        {
            id: "tick",
            type: "input",
            position: {
                x: 250,
                y: 0,
            },
            data: {
                name: "tick",
                label: "tick",
                fullName: "plugin.common.timer.tick",
                dirty: false,
                portType: "Source",
            },
        },
        {
            id: "broadcast",
            type: "default",
            position: {
                x: 250,
                y: 150,
            },
            data: {
                name: "broadcast",
                label: "broadcast",
                fullName: "plugin.common.fanout.broadcast",
                dirty: false,
                portType: "Flow",
            },
        },
        {
            id: "last",
            type: "output",
            position: {
                x: 250,
                y: 250,
            },
            data: {
                name: "last",
                label: "last",
                fullName: "plugin.common.collect.last",
                dirty: false,
                portType: "Sink",
            },
        },
        {
            id: "count",
            type: "output",
            position: {
                x: 500,
                y: 250,
            },
            data: {
                name: "count",
                label: "count",
                fullName: "plugin.common.collect.count",
                dirty: false,
                portType: "Sink",
            },
        },
        {
            id: "out",
            type: "output",
            position: {
                x: 750,
                y: 250,
            },
            data: {
                name: "stdout",
                label: "stdout",
                fullName: "plugin.common.stdio.stdout",
                dirty: false,
                portType: "Sink",
            },
        },
        {
            id: "link-tick-broadcast",
            source: "tick",
            target: "broadcast",
        },
        {
            id: "link-broadcast-last",
            source: "broadcast",
            target: "last",
        },
        {
            id: "link-broadcast-count",
            source: "broadcast",
            target: "count",
        },
        {
            id: "link-broadcast-out",
            source: "broadcast",
            target: "out",
        },
    ],
} as ChartData;

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
            height={props.height}
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

    return (
        <Chart
            data={_testChartData /*props.state.chart*/}
            dispatch={props.dispatch}
            height={props.height}
        />
    );
};

const LeftPane = styled(Box)(({ theme }) => ({
    flexGrow: 1,
    zIndex: 990,
    marginRight: theme.spacing(3),
    height: "100%",
}));

const OuterResizable = styled(Resizable)(({ theme }) => ({
    position: "relative",
    display: "flex",
    flexGrow: 1,
}));

const RightResizable = styled(Resizable)(({ theme }) => ({
    position: "relative",
}));

const BottomPane = styled(Box)(({ theme }) => ({
    bottom: 0,
    left: 0,
    width: "100%",
    backgroundColor: theme.palette.background.default,
    zIndex: 10000,
}));

export const GraphEdit = () => {
    const { name } = useParams<{ name: string }>();
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

    const [rightPaneSize, setRightPaneSize] = React.useState(() => {
        return { width: 240 };
    });

    const [contentSize, setContentSize] = React.useState(() => {
        return { content: 600, bottom: 180 };
    });

    const onRightPaneResize = (_event, { size }) => {
        setRightPaneSize({ width: size.width });
    };

    const onBottomResize = (_event, { size }) => {
        setContentSize((prev) => {
            const delta = prev.content - size.height;
            return {
                content: size.height,
                bottom: prev.bottom + delta,
            };
        });
    };

    const onTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: number) => {
            setTabNumber(newValue);
        },
        [state.services]
    );

    return (
        <Stack>
            <OuterResizable
                width={Infinity}
                height={contentSize.content}
                onResize={onBottomResize}
                handle={<span className={classes.resizeHandleBottom} />}
                resizeHandles={["s"]}
            >
                <Box
                    sx={{
                        display: "flex",
                        height: contentSize.content,
                        zIndex: 1200,
                    }}
                >
                    <LeftPane>
                        <Typography
                            sx={{ marginBottom: 2 }}
                            variant="h6"
                            component="div"
                        >
                            {name}
                        </Typography>
                        <Stack spacing={1} sx={{ height: "100%" }}>
                            <Stack direction="row" spacing={2}>
                                <IconButton
                                    aria-label="refresh"
                                    onClick={onChartRefleshClick}
                                    size="large"
                                >
                                    <RefreshIcon />
                                </IconButton>
                            </Stack>
                            <Paper elevation={2} sx={{ height: "100%" }}>
                                <Box
                                    p={2}
                                    sx={{ height: "100%", display: "flex" }}
                                >
                                    <React.Suspense
                                        fallback={<CircularProgress />}
                                    >
                                        <ChartSuspense
                                            state={state}
                                            dispatch={dispatch}
                                            serviceResource={serviceResource}
                                            graphResource={graphResource}
                                        />
                                    </React.Suspense>
                                </Box>
                            </Paper>
                        </Stack>
                    </LeftPane>
                    <RightResizable
                        width={rightPaneSize.width}
                        height={Infinity}
                        onResize={onRightPaneResize}
                        handle={<span className={classes.resizeHandle} />}
                        resizeHandles={["w"]}
                    >
                        <Box
                            sx={{ width: rightPaneSize.width, height: "100%" }}
                        >
                            <Tabs
                                value={tabNumber}
                                onChange={onTabChange}
                                aria-label="tabs"
                                variant="scrollable"
                                scrollButtons="auto"
                            >
                                <Tab label="Services" />
                            </Tabs>
                            <React.Suspense fallback={<CircularProgress />}>
                                <SidebarSuspense
                                    state={state}
                                    dispatch={dispatch}
                                    serviceResource={serviceResource}
                                    graphResource={graphResource}
                                />
                            </React.Suspense>
                        </Box>
                    </RightResizable>
                </Box>
            </OuterResizable>
            <BottomPane sx={{ height: contentSize.bottom }}>
                <span>aiueo</span>
            </BottomPane>
            <NodeEditor
                state={state}
                dispatch={dispatch}
                open={!!state.edit.id}
            />
        </Stack>
    );
};

export default GraphEdit;
