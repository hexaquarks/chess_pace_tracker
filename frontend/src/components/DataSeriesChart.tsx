import React, { useEffect } from 'react';
import ApexCharts from 'apexcharts';
import { TrendChartDatum } from '../services/apiService';

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

    const tooltipFormatter = ({ series, seriesIndex, dataPointIndex, w }: any) => {
        const value = series[seriesIndex][dataPointIndex];
        const status = winStatus[dataPointIndex];
        const gameNumber = gameNumbers[dataPointIndex];
    
        return (
          '<div class="arrow_box" style="padding: 10px; text-align: center;">' +
          '<span style="font-size: 16px; font-weight: bold;">' + signedTimeFormatter(value) + '</span>' +
          '<br>' + 
          '<span>' + status + '</span>' +
          '<br>' +
          '<span>' + 'Game ' + gameNumber + '</span>' +
          '</div>'
        );
    }

    const signedTimeFormatter = (time: number) => { 
        return (time > 0.0 ? '+' + time : time)+ 's'
    }

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
                height: 300,
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
                custom: tooltipFormatter
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
                style: {
                  fontSize: '12px',
                  fontWeight: 'bold',
                },
                background: {
                  enabled: true,
                  foreColor: '#fff',
                  borderRadius: 2,
                  padding: 4,
                  opacity: 0.9,
                  borderWidth: 1,
                  borderColor: 'black'
                },
                formatter: signedTimeFormatter
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
                labels: {
                  show: false,
                },
                axisBorder: {
                  show: false,
                },
                axisTicks: {
                  show: false,
                },
                tooltip: {
                  enabled: false, 
                },
                crosshairs: {
                    show: false
                  }
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
