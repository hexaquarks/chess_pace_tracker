import React, { useEffect } from 'react';
import ApexCharts from 'apexcharts';

import { ResponseInformation } from '../../services/apiService';

interface FlagPanelProps {
    userFlagCount: number,
    opponentsFlagCount: number,
    totalGamesConsidered: number
}

export const extractFlagginInfoFromResponse = (response: ResponseInformation): FlagPanelProps => {
  let userFlagCount: number = response.players_flag_counts[0];
  let opponentsFlagCount: number = response.players_flag_counts[1];
  let totalGamesConsidered: number = response.trend_chart_data.length;

  return { userFlagCount, opponentsFlagCount, totalGamesConsidered };
};

const renderFlagChart = ({ userFlagCount, opponentsFlagCount, totalGamesConsidered }: FlagPanelProps) => {
  const options = {
    series: [{
      data: [userFlagCount, opponentsFlagCount, totalGamesConsidered]
    }],
    chart: {
      type: 'bar',
      height: 200,
      toolbar: {
        show: false // Remove burger menu
      }
    },
    plotOptions: {
      bar: {
        horizontal: true,
        barHeight: '50%', // Thick bars
      }
    },
    colors: ['#f87171', '#4ade80', '#4ade80'],
    legend: {
      show: false
    },
    grid: {
      show: false
    },
    xaxis: {
      categories: ['flagged opponent', 'opponent flagged', 'total games'],
      labels: {
        show: false // Remove bottom x ticks
      }
    },
    yaxis: {
      labels: {
        style: {
          colors: '#FFFFFF' // Make y-axis labels white
        },
        rotate: -45 // Rotate y-axis labels
      },
      reversed: true
    },
    tooltip: {
      enabled: false
    }
  };

  const chart = new ApexCharts(document.querySelector("#flagging-chart"), options);
  chart.render();

  return () => {
    chart.destroy();
  }
};


export const FlagPanel: React.FC<FlagPanelProps> = ({ 
  userFlagCount, 
  opponentsFlagCount, 
  totalGamesConsidered 
}) => {
  useEffect(() => {
    return renderFlagChart({ userFlagCount, opponentsFlagCount, totalGamesConsidered });
  }, []);

  return (
    <div id="flagging-chart"></div>
  );
};