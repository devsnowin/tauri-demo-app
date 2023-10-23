import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function formatDate(dateString: string): string {
    const months = [
        'Jan',
        'Feb',
        'Mar',
        'Apr',
        'May',
        'Jun',
        'Jul',
        'Aug',
        'Sep',
        'Oct',
        'Nov',
        'Dec',
    ];

    const dateTime = new Date(dateString);
    if (isNaN(dateTime.getTime())) {
        return 'Invalid Date';
    }

    const options = {
        hour: 'numeric',
        minute: 'numeric',
        hour12: true,
    } satisfies Intl.DateTimeFormatOptions;

    const formattedDate = dateTime.toLocaleString('en-US', options);
    const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const dayName = dayNames[dateTime.getUTCDay()];
    const dayOfMonth = dateTime.getUTCDate();
    const month = months[dateTime.getUTCMonth()];

    return `${dayName} ${dayOfMonth} ${month} ${formattedDate}`;
}
