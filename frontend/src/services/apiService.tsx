interface RequestInformation {
  username: string;
  games_count: number;
  game_mode: string;
  user_color: string;
}

export const sendDataToBackend = async (
  username: string,
  gamesCount: number,
  gameMode: string,
  userColor: string) => {
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

    const data = await response.json();
    console.log(data);
  } catch (error) {
    console.error('Error sending data to backend', error);
  }
};
