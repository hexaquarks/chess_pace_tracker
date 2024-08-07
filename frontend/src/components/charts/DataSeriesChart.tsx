import React, { useEffect, useRef } from 'react';
import ApexCharts from 'apexcharts';
import { TrendChartDatum } from '../../services/apiService';

const XS_SCREEN_WIDTH = 480;

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
  // useRef to ensure that the chart's stored as a single instance across potential re-renders.
  // The instance storage is necessary here because the chart's height is updated on window resize.
  const chartRef = useRef<ApexCharts | null>(null);
  const getChartHeight = () => window.innerWidth <= XS_SCREEN_WIDTH ? '200px' : '300px';
  const getLabelSize = () => window.innerWidth <= XS_SCREEN_WIDTH ? '9px' : '12px';
  const getStrokeWidth = () => window.innerWidth <= XS_SCREEN_WIDTH ? 3 : 6;

  const tooltipFormatter = ({ series, seriesIndex, dataPointIndex, w }: any) => {
    const value = series[seriesIndex][dataPointIndex];
    const status = winStatus[dataPointIndex];
    const gameNumber = gameNumbers[dataPointIndex];

    return (
      `<div class="apexcharts-tooltip-title" style="font-size: 16px; font-weight: bold; padding: 4px; margin-bottom: -5px">${signedTimeFormatter(value)}</div>` +
      `<div class="apexcharts-tooltip-series-group" style="padding: 10px; text-align: left; display: block;">` +
      `<span class="apexcharts-tooltip-text" style="display: block;">Status: <span style="font-weight: bold;">` + status + `</span></span>` +
      `<span class="apexcharts-tooltip-text" style="display: block;">Game: <span style="font-weight: bold;">` + gameNumber + `</span></span>` +
      `</div>`
    );
  }

  const signedTimeFormatter = (time: number) => {
    return (time > 0.0 ? '+' + time : time) + 's'
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
        height: getChartHeight(),
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
        custom: tooltipFormatter,
        theme: "dark"
      },
      legend: {
        show: false
      },
      fill: {
        type: "gradient",
        gradient: {
          opacityFrom: 0,
          opacityTo: 0.55,
          gradientToColors: ["#1C64F2"],
        }
      },
      dataLabels: {
        style: {
          fontSize: getLabelSize(),
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
        width: getStrokeWidth(),
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
        labels: {
          show: false,
          align: 'left'
        }
      },
      annotations: {
        yaxis: [{
          y: 0,
          borderColor: 'rgba(255, 255, 255, 0.3)',
          strokeDashArray: 5,
          label: {
            borderColor: 'rgba(255, 255, 255, 0.3)',
            style: {
              color: '#fff',
              background: 'transparent'
            },
            text: '0.0s',
            position: 'left',
            offsetX: -10,
            offsetY: 7
          }
        }]
      }
    }

    chartRef.current = new ApexCharts(document.querySelector("#data-series-chart"), options);
    chartRef.current.render().then(() => {
      // We explicitly set the svg (chart) generated's overflow property 
      // to visible, otherwise some marker faces will be cropped at the 
      // beggining and end of the chart area.
      const svgElement = document.querySelector("#data-series-chart svg");
      if (svgElement instanceof SVGSVGElement) {
        svgElement.style.overflow = 'visible'
      }
    });

    // Window resize handler. A chart ref is used to update the chart's height.
    const resizeListener = () => {
      chartRef.current?.updateOptions({
        chart: {
          height: getChartHeight()
        },
        dataLabels: {
          style: {
            fontSize: getLabelSize(),
          },
        },
        stroke: {
          width: getStrokeWidth(),
        }
      });
    };

    window.addEventListener('resize', resizeListener);

    // Clean up function to avoid memory leaks.
    return () => {
      window.removeEventListener('resize', resizeListener);
      chartRef.current?.destroy();
    };
  }, []);

  return (
    <div id="data-series-chart">
    </div>
  );
};
