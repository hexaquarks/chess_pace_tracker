interface RequestInformation {
  username: string;
  games_count: number;
  game_mode: string;
  user_color: string;
}

export enum MessageInformationAssessment {
  Positive,
  Neutral,
  Negative
}

export interface TrendChartDatum {
  time_differential: number,
  win_status: string,
  game_number: number
}

interface ResponseInformationInternal {
  time: number; 
  explanation_message: [string, MessageInformationAssessment]
  games_with_errors: Array<[number, string]>
  trend_chart_data: [TrendChartDatum]
  player_win_rate_in_fetched_games: number
  players_flag_counts: [number, number]
}

export interface ResponseInformation {
  time: number; 
  explanation_message: [string, MessageInformationAssessment]
  games_with_errors: Array<[number, string]>
  trend_chart_data: [TrendChartDatum]
  player_win_rate_in_fetched_games: number
  players_flag_counts: [number, number]
  unique_key: number
}

export const sendDataToBackend = async (
  username: string,
  gamesCount: number,
  gameMode: string,
  userColor: string): Promise<ResponseInformation> => {
  try {
    const payload: RequestInformation = {
      username,
      games_count: gamesCount,
      game_mode: gameMode,
      user_color: userColor
    };
    console.log('Sending:', payload);

    const response = await fetch('http://localhost:8000/fetch-chess-data', {
      method: 'POST',
      body: JSON.stringify(payload),
      headers: {
        'Content-Type': 'application/json',
        "x-requested-by": "frontend"
      },
    });

    if (!response.ok) {
      const errorResponse = await response.json();
      throw new Error(errorResponse.error || `HTTP error! status: ${response.status}`);
    }

    const data: ResponseInformationInternal = await response.json();
    console.log(data);
    if (!data.games_with_errors) {
      console.log("its empt");
    }

    const uniqueData: ResponseInformation = {...data, unique_key: Date.now() }
    
    return uniqueData;

  } catch (error) {
    console.error('Error sending data to backend', error);

    throw error;
  }
};
