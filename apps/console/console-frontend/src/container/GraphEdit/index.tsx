import * as React from "react";
import { useParams } from "react-router-dom";
import { Theme, styled } from "@mui/material/styles";
import createStyles from "@mui/styles/createStyles";
import makeStyles from "@mui/styles/makeStyles";
import RefreshIcon from "@mui/icons-material/Refresh";
import {
    Box,
    Tab,
    Tabs,
    IconButton,
    Paper,
    Stack,
    Typography,
    Divider,
} from "@mui/material";
import { Chart } from "./chart";
import { reducer, initialState, ServiceState } from "../../modules/graphEdit";
import { fetchServices, fetchGraph } from "../../modules/api/toy-api";
import { ChartData } from "../../modules/graphEdit/types";
import CircularProgress from "../../components/progress/CircularProgress";
import { Resizable } from "react-resizable";
import { NodeEditor } from "./NodeEditor";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        resizeHandle: {
            position: "absolute",
            width: "2px",
            height: "100%",
            backgroundColor: theme.palette.divider,
            opacity: "0.75",
            top: "0",
            cursor: "ew-resize",
        },
        resizeHandleBottom: {
            position: "absolute",
            width: "100%",
            height: "2px",
            backgroundColor: theme.palette.divider,
            opacity: "0.75",
            bottom: "0",
            cursor: "ns-resize",
            zIndex: 11000,
        },
    })
);

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

    const [serviceResource, setServiceResource] = React.useState(() =>
        fetchServices()
    );
    const [graphResource, setGraphResource] = React.useState(() =>
        fetchGraph(name)
    );
    const [state, dispatch] = React.useReducer(reducer, initialState);
    const [tabNumber, setTabNumber] = React.useState(0);
    const [bottomTabNumber, setBottomTabNumber] = React.useState(0);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const [rightPaneSize, setRightPaneSize] = React.useState(() => {
        return { width: 240 };
    });

    const [contentSize, setContentSize] = React.useState(() => {
        return { content: 500, bottom: 250 };
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

    const onBottomTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: number) => {
            setBottomTabNumber(newValue);
        },
        []
    );

    return (
        <Stack spacing={0}>
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
                        <Stack spacing={0} sx={{ height: "100%" }}>
                            <Paper sx={{ height: "100%", width: "100%", p: 1 }}>
                                <Box sx={{ paddingLeft: 1, paddingBottom: 1 }}>
                                    <Typography variant="h6">{name}</Typography>
                                </Box>
                                <Divider />
                                <Box sx={{ width: "100%" }}>
                                    <IconButton
                                        aria-label="refresh"
                                        onClick={onChartRefleshClick}
                                        size="large"
                                    >
                                        <RefreshIcon />
                                    </IconButton>
                                </Box>
                                <Divider />
                                <Box
                                    p={2}
                                    sx={{ height: "100%", display: "flex" }}
                                >
                                    <React.Suspense
                                        fallback={<CircularProgress />}
                                    >
                                        <Chart
                                            data={
                                                /*_testChartData*/ state.chart
                                            }
                                            graphResource={graphResource}
                                            serviceResource={serviceResource}
                                            dispatch={dispatch}
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
                            sx={{
                                width: rightPaneSize.width,
                                height: "100%",
                                overflowY: "auto",
                            }}
                        >
                            <Tabs
                                value={tabNumber}
                                onChange={onTabChange}
                                aria-label="tabs"
                                variant="scrollable"
                                scrollButtons="auto"
                            >
                                <Tab label="config" />
                            </Tabs>
                            <NodeEditor state={state} dispatch={dispatch} />
                        </Box>
                    </RightResizable>
                </Box>
            </OuterResizable>
            <BottomPane sx={{ height: contentSize.bottom }}>
                <Paper
                    variant="outlined"
                    sx={{ height: "100%", width: "100%", p: 1 }}
                >
                    <Tabs
                        value={bottomTabNumber}
                        onChange={onBottomTabChange}
                        aria-label="tabs"
                        variant="scrollable"
                        scrollButtons="auto"
                    >
                        <Tab label="CONSOLE" />
                        <Tab label="????" />
                    </Tabs>
                </Paper>
            </BottomPane>
        </Stack>
    );
};

export default GraphEdit;
