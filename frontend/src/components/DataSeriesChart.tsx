import React, { useEffect } from 'react';
import ApexCharts from 'apexcharts';
import { ResponseInformation, TrendChartDatum } from '../services/apiService';

interface DataSeriesChartProps {
    times: number[],
    winStatus: string[],
    gameNumbers: number[],
}

export const convertTrendChartData = (data: [TrendChartDatum]): DataSeriesChartProps => {
    let times: number[] = [];
    let winStatus: string[] = [];
    let gameNumbers: number[] = [];

    data.forEach((item) => {
        times.push(item.time_differential);
        winStatus.push(item.win_status);
        gameNumbers.push(item.game_number);
    });

    return { times, winStatus, gameNumbers };
};

export const DataSeriesChart: React.FC<DataSeriesChartProps> = ({ times, winStatus, gameNumbers }) => {
    useEffect(() => {
        let options: ApexCharts.ApexOptions = {
            series: [
                {
                    name: "",
                    data: times,
                    color: "#1A56DB",
                }
            ],
            chart: {
                height: "100%",
                type: "area",
                fontFamily: "Inter, sans-serif",
                dropShadow: {
                    enabled: false,
                },
                toolbar: {
                    show: false,
                },
            },
            tooltip: {
                enabled: true,
                x: {
                    show: false,
                },
            },
            legend: {
                show: false
            },
            fill: {
                type: "gradient",
                gradient: {
                    opacityFrom: 0.55,
                    opacityTo: 0,
                    shade: "#1C64F2",
                    gradientToColors: ["#1C64F2"],
                },
            },
            dataLabels: {
                enabled: false,
            },
            stroke: {
                width: 6,
            },
            grid: {
                show: false,
                strokeDashArray: 4,
                padding: {
                    left: 2,
                    right: 2,
                    top: 0
                },
            },
            xaxis: {
                categories: winStatus,
                labels: {
                    show: false,
                },
                axisBorder: {
                    show: false,
                },
                axisTicks: {
                    show: false,
                },
            },
            yaxis: {
                show: false,
                labels: {
                    formatter: function (value: number) {
                        return (value > 0.0 ? '+' + value : value)+ 's';
                    }
                }
            },
        }

        const chart = new ApexCharts(document.querySelector("#data-series-chart"), options);
        chart.render();

        return () => {
            chart.destroy();
        };
    }, []);

    return (
        <div id="data-series-chart">
        </div>
    );
};
