import * as React from "react";
import {
    DataGrid,
    GridColumns,
    GridRowId,
    GridActionsCellItem,
} from "@mui/x-data-grid";
import CircularProgress from "../../components/progress/CircularProgress";
import {
    RbacClient,
    Role,
    RoleList,
    ErrorMessage,
    useFetch,
} from "../../modules/api";
import { Box, Tab, Tabs, Stack, Link } from "@mui/material";
import { RuleList } from "./RuleList";

const userColumns: GridColumns<{ name: string; role: string; note: string }> = [
    { field: "name", headerName: "name", width: 150 },
    { field: "type", headerName: "type", width: 150 },
    { field: "role", headerName: "role", width: 150 },
    {
        field: "note",
        headerName: "note",
        width: 150,
    },
];

const roleColumns = (props: GridProps): GridColumns<Role> => {
    return [
        {
            field: "name",
            headerName: "name",
            type: "string",
            width: 150,
            renderCell: (params) => (
                <Link component="button" onClick={props.onShowRules(params.id)}>
                    {params.value}
                </Link>
            ),
        },
        {
            field: "note",
            headerName: "note",
            type: "string",
            width: 150,
        },
        {
            field: "actions",
            type: "actions",
            getActions: (params) => [
                <GridActionsCellItem
                    onClick={props.onShowRules(params.id)}
                    label="ShowRules"
                    showInMenu
                />,
            ],
        },
    ];
};

type TabType = "User" | "Role";

interface GridProps {
    tp: TabType;
    onShowRules: (id: GridRowId) => () => void;
}

const UserGrid = (prop: GridProps): JSX.Element => {
    const { tp } = prop;
    return (
        <DataGrid
            getRowId={(r) => r.name}
            rows={[]}
            columns={userColumns}
            checkboxSelection
            disableSelectionOnClick
            experimentalFeatures={{ newEditingApi: true }}
        />
    );
};

const RoleGrid = (prop: GridProps): JSX.Element => {
    const columns = roleColumns(prop);
    const roles = useFetch(["fetchRoles"], RbacClient.fetchRoles);
    const items = roles && roles.isSuccess() ? roles.value.items : [];
    return (
        <DataGrid
            getRowId={(r) => r.name}
            rows={items}
            columns={columns}
            checkboxSelection
            disableSelectionOnClick
            experimentalFeatures={{ newEditingApi: true }}
        />
    );
};

export const ManageAuth = () => {
    const [tabLabel, setTabLabel] = React.useState<TabType>("User");
    const [ruleListOpen, setRuleListOpen] = React.useState({
        open: false,
        name: null,
    });

    const onTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: TabType) => {
            setTabLabel(newValue);
        },
        []
    );

    const onShowRules = React.useCallback(
        (id: GridRowId) => () => {
            setRuleListOpen({ open: true, name: id });
        },
        []
    );
    const onRuleListClose = React.useCallback(() => {
        setRuleListOpen({ open: false, name: null });
    }, []);

    return (
        <Box sx={{ height: 400, width: "100%", display: "flex" }}>
            <Stack direction="column" spacing={2} sx={{ width: "100%" }}>
                <Tabs
                    value={tabLabel}
                    onChange={onTabChange}
                    aria-label="tabs"
                    variant="scrollable"
                    scrollButtons="auto"
                >
                    <Tab label="User" value="User" />
                    <Tab label="Role" value="Role" />
                </Tabs>

                <Box sx={{ width: "100%", flexGrow: 1 }}>
                    <React.Suspense fallback={<CircularProgress />}>
                        {tabLabel == "Role" ? (
                            <RoleGrid tp={tabLabel} onShowRules={onShowRules} />
                        ) : (
                            <UserGrid tp={tabLabel} onShowRules={onShowRules} />
                        )}
                    </React.Suspense>
                    <React.Suspense fallback={<CircularProgress />}>
                        <RuleList
                            name={ruleListOpen.name}
                            open={ruleListOpen.open}
                            onClose={onRuleListClose}
                        />
                    </React.Suspense>
                </Box>
            </Stack>
        </Box>
    );
};
