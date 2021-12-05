import * as React from "react";
import { Theme } from "@mui/material/styles";
import createStyles from '@mui/styles/createStyles';
import makeStyles from '@mui/styles/makeStyles';
import Chip from "@mui/material/Chip";
import Paper from "@mui/material/Paper";

export interface LabelsProps {
    labels: { key: string; display: string }[];
}

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            display: "flex",
            justifyContent: "left",
            flexWrap: "wrap",
            listStyle: "none",
            padding: theme.spacing(0.5),
            margin: 0,
            backgroundColor: "inherit",
        },
        chip: {
            margin: theme.spacing(0.5),
        },
    })
);

export const LabelChips = (props: LabelsProps) => {
    const classes = useStyles();
    return (
        <Paper component="ul" className={classes.root} elevation={0}>
            {props.labels.map((data) => {
                return (
                    <li key={data.key}>
                        <Chip label={data.display} className={classes.chip} />
                    </li>
                );
            })}
        </Paper>
    );
};
