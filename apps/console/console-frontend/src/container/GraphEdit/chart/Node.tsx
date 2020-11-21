import * as React from "react";
import { makeStyles, createStyles, Theme } from "@material-ui/core/styles";
import Card from "@material-ui/core/Card";
import { INodeInnerDefaultProps, INode } from "@mrblenny/react-flow-chart";
import { ServiceCardHeader } from "./ServiceCardHeader";
import IconButton from "@material-ui/core/IconButton";
import EditIcon from "@material-ui/icons/Edit";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const useStyles = makeStyles((_: Theme) =>
    createStyles({
        card: {
            maxWidth: 300,
        },
        row: {
            display: "flex",
            justifyContent: "space-between",
        },
    })
);

export type NodeProps = INodeInnerDefaultProps;

const getProperties = (node: INode) => {
    if (node.properties) {
        return {
            title: node.properties.name,
            dirty: node.properties.dirty,
            portType: node.properties.portType,
        };
    } else {
        return null;
    }
};

export const Node = React.memo((props: NodeProps) => {
    const classes = useStyles();
    const dispatch = props.config.nodeProps.dispatch;
    const onEdit = React.useCallback(() => {
        if (dispatch) {
            dispatch({
                type: "StartEditNode",
                payload: props.node.id,
            });
        }
    }, [dispatch, props.node.id]);

    return (
        <Card className={classes.card}>
            <div className={classes.row}>
                <ServiceCardHeader {...getProperties(props.node)} />
                <IconButton aria-label="edit" onClick={onEdit}>
                    <EditIcon fontSize="small" />
                </IconButton>
            </div>
        </Card>
    );
});
