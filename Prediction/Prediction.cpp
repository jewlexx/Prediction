#include <ctime>
#include <iostream>
#include <string>

using namespace std;

int rand_range(const int high)
{
	srand(time(nullptr));
	return rand() % high;
}

int main()
{
	string placeholder;
	cout << "What would you like to know? ";
	cin >> placeholder;
	string ball_options[20] = {
		"It is certain",
		"It is decidedly so",
		"Without a doubt",
		"Yes - definitely",
		"You may rely on it",
		"As I see it, yes",
		"Most likely",
		"Outlook good",
		"Yes",
		"Signs point to yes",
		"Reply hazy, try again",
		"Ask again later",
		"Better not tell you now",
		"Cannot predict now",
		"Concentrate and ask again",
		"Don't count on it",
		"My reply is no",
		"My sources say no",
		"Outlook not so good",
		"Very doubtful",
	};

	cout << "Magic 8 ball says: " << ball_options[rand_range(19)] << endl << endl;

	system("pause");
	return 0;
}
