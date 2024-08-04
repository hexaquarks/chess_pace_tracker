import React, { useEffect } from 'react';
import ApexCharts from 'apexcharts';

interface WinRateDonutChartProps {
    winRate: number;
}

const WinRateDonutChart: React.FC<WinRateDonutChartProps> = ({ winRate }) => {
    const getPercentageWinAndLoss = (winRate: number) => {
        return [winRate * 100, 100 - winRate * 100]
    }
    useEffect(() => {
        const options: ApexCharts.ApexOptions = {
            series: getPercentageWinAndLoss(winRate),
            chart: {
                type: 'donut',
                toolbar: {
                    show: false
                }
            },
            colors: ['#10B981', '#EF4444'],
            labels: ['Win Rate', 'Loss Rate'],
            stroke: {
                colors: ["transparent"],
                lineCap: undefined,
            },
            dataLabels: {
                enabled: false
            },
            legend: {
                show: true,
                showForSingleSeries: true,
                showForNullSeries: true,
                showForZeroSeries: true,
                position: 'bottom'
            },
            tooltip: {
                y: {
                    formatter: function (val: any) {
                        return val + '%';
                    }
                }
            },
            plotOptions: {
                pie: {
                    donut: {
                        labels: {
                            show: true,
                            name: {
                                show: true,
                                fontFamily: "Inter, sans-serif",
                                offsetY: 20,
                                color: 'red'
                            },
                            total: {
                                showAlways: true,
                                show: true,
                                label: "Win ratio",
                                fontFamily: "Inter, sans-serif",
                                color: 'red',
                                formatter: function (w: any) {
                                    return w.globals.series[0].toFixed(0) + "%"
                                },
                            },
                            value: {
                                show: true,
                                fontFamily: "Inter, sans-serif",
                                offsetY: -20,
                                color: 'red',
                                formatter: function (value: String) {
                                    return value + "k"
                                },
                            }
                        },
                        size: "80%"
                    }
                }
            },
            responsive: [
                {
                    options: {
                        chart: {
                            width: 200
                        }
                    }
                }
            ]
        };

        const chart = new ApexCharts(document.querySelector("#win-rate-chart"), options);
        chart.render();

        return () => {
            chart.destroy();
        };
    }, [winRate]);

    return (
        <div id="win-rate-chart">
        </div>
    );
};

export default WinRateDonutChart;
