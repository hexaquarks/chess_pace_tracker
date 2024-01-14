interface RequestInformation {
  username: string;
  games_count: number;
  game_mode: string;
  user_color: string;
}

enum MessageInformationAssessment {
  Positive,
  Neutral,
  Negative
}

export interface ResponseInformation {
  time: number; 
  explanation_message: [string, MessageInformationAssessment]
  games_with_error: Array<[number, string]>
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
      },
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data: ResponseInformation = await response.json();
    console.log(data);
    
    return data;

  } catch (error) {
    console.error('Error sending data to backend', error);

    throw error;
  }
};
