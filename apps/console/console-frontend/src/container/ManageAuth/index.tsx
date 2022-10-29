import * as React from "react";
import {
    DataGrid,
    GridColumns,
    GridRowId,
    GridActionsCellItem,
} from "@mui/x-data-grid";
import CircularProgress from "../../components/progress/CircularProgress";
import { RbacClient, Role, RoleList } from "../../modules/api";
import { Resource } from "../../modules/common";
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
    rolesResource: Resource<RoleList>;
    tp: TabType;

    onShowRules: (id: GridRowId) => () => void;
}
const Grid = (prop: GridProps): JSX.Element => {
    const { rolesResource, tp } = prop;
    switch (tp) {
        case "User":
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
        case "Role":
            const roles = rolesResource.read();
            const items = roles ? roles.items : [];
            const columns = roleColumns(prop);
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
    }
};

export const ManageAuth = () => {
    const [tabLabel, setTabLabel] = React.useState<TabType>("User");
    const [rolesResource, setRolesResource] = React.useState(() =>
        RbacClient.fetchRoles()
    );
    const [ruleListOpen, setRuleListOpen] = React.useState({
        open: false,
        name: null,
        resource: null,
    });

    const onTabChange = React.useCallback(
        (_event: React.ChangeEvent<{}>, newValue: TabType) => {
            setTabLabel(newValue);
        },
        []
    );

    const onShowRules = React.useCallback(
        (id: GridRowId) => () => {
            const resource = RbacClient.fetchRole(id.toString());
            setRuleListOpen({ open: true, name: id, resource });
        },
        []
    );
    const onRuleListClose = React.useCallback(() => {
        setRuleListOpen({ open: false, name: null, resource: null });
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
                        <Grid
                            rolesResource={rolesResource}
                            tp={tabLabel}
                            onShowRules={onShowRules}
                        />
                    </React.Suspense>
                    <React.Suspense fallback={<CircularProgress />}>
                        <RuleList
                            resource={ruleListOpen.resource}
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
