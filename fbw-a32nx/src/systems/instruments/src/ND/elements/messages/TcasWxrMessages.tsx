// Copyright (c) 2021-2023 FlyByWire Simulations
//
// SPDX-License-Identifier: GPL-3.0

import React, { FC } from 'react';
import { Layer, useSimVar } from '@flybywiresim/fbw-sdk';
import { EfisNdMode, TcasWxrMessage } from '@shared/NavigationDisplay';

/*
Messages in priority order, from 1-12 (full set with ATSAW and nice weather radar)
[  TCAS (amber)   | WEATHER AHEAD (amber) ]
[  TCAS (amber)   |     ADS-B (amber)     ]
[  TCAS (amber)   |   ADS-B ONLY (white)  ]
[  TCAS (amber)   |                       ]
[   XX.YNM+NN^    |       XX.YNM+NN^      ]
[   XX.YNM+NN^    |     ADS-B (amber)     ]
[   XX.YNM+NN^    |                       ]
[ TA ONLY (white) | WEATHER AHEAD (amber) ]
[ TA ONLY (white) |     ADS-B (amber)     ]
[ TA ONLY (white) |                       ]
[                 |     ADS-B (amber)     ]
*/

export const TcasWxrMessages: FC<{ modeIndex: EfisNdMode}> = ({ modeIndex }) => {
    // TODO get data and decide what to display

    let leftMessage: TcasWxrMessage | undefined;
    let rightMessage: TcasWxrMessage | undefined;

    const [tcasOnly] = useSimVar('L:A32NX_TCAS_TA_ONLY', 'boolean', 200);
    const [tcasFault] = useSimVar('L:A32NX_TCAS_FAULT', 'boolean', 200);

    if (tcasFault) {
        leftMessage = { text: 'TCAS', color: 'Amber' };
    } else if (tcasOnly) {
        leftMessage = { text: 'TA ONLY', color: 'White' };
    }

    if (modeIndex !== EfisNdMode.ARC && modeIndex !== EfisNdMode.ROSE_NAV && modeIndex !== EfisNdMode.ROSE_VOR && modeIndex !== EfisNdMode.ROSE_ILS || (!leftMessage && !rightMessage)) {
        return null;
    }

    const y = (modeIndex === EfisNdMode.ROSE_VOR || modeIndex === EfisNdMode.ROSE_ILS) ? 713 : 684;

    return (
        <Layer x={164} y={y}>
            { /* we fill/mask the map under both message boxes, per IRL refs */ }
            { (modeIndex === EfisNdMode.ARC || modeIndex === EfisNdMode.ROSE_NAV) && (
                <rect x={0} y={0} width={440} height={59} className="BackgroundFill" stroke="none" />
            )}

            <rect x={0} y={0} width={440} height={30} className="White BackgroundFill" strokeWidth={1.75} />

            { (leftMessage) && (
                <text
                    x={8}
                    y={25}
                    className={`${leftMessage.color}`}
                    textAnchor="start"
                    fontSize={25}
                >
                    {leftMessage.text}
                </text>
            )}

            { (rightMessage) && (
                <text
                    x={425}
                    y={25}
                    className={`${rightMessage.color}`}
                    textAnchor="end"
                    fontSize={25}
                >
                    {rightMessage.text}
                </text>
            )}
        </Layer>
    );
};
