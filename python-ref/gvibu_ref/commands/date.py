"""date: print or set the system date and time."""

import sys
import datetime
import re
import time


WEEKDAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
WEEKDAYS_ABBR = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
MONTHS = ["January", "February", "March", "April", "May", "June",
          "July", "August", "September", "October", "November", "December"]
MONTHS_ABBR = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
               "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]


def run(args: list[str]) -> int:
    utc = False
    rfc2822 = False
    iso8601 = False
    format_str = None
    date_string = None
    parsing_date_arg = False
    i = 0

    while i < len(args):
        arg = args[i]
        if parsing_date_arg:
            date_string = arg
            parsing_date_arg = False
        elif arg in ("-d", "--date"):
            parsing_date_arg = True
        elif arg in ("-u", "--utc", "--universal"):
            utc = True
        elif arg in ("-R", "--rfc-2822", "--rfc-email"):
            rfc2822 = True
        elif arg in ("-I", "--iso-8601"):
            iso8601 = True
        elif arg.startswith("+"):
            format_str = arg[1:]
        elif arg.startswith("-") and len(arg) > 1:
            print(f"date: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            print(f"date: extra operand '{arg}'", file=sys.stderr)
            return 1
        i += 1

    if parsing_date_arg:
        print("date: option requires an argument -- 'd'", file=sys.stderr)
        return 1

    if utc:
        now = datetime.datetime.now(datetime.timezone.utc)
    else:
        now = datetime.datetime.now(datetime.timezone(datetime.timedelta(0)))

    if date_string is not None:
        try:
            now = parse_date_argument(date_string, now)
        except (ValueError, OverflowError) as error:
            print(f"date: invalid date '{date_string}'", file=sys.stderr)
            return 1

    local_now = datetime.datetime.now()

    if rfc2822:
        print(now.strftime("%a, %d %b %Y %H:%M:%S %z"))
        return 0

    if iso8601:
        print(now.strftime("%Y-%m-%d"))
        return 0

    if format_str is not None:
        print(format_custom(now, format_str))
        return 0

    dt = local_now if date_string is None else now
    wd = WEEKDAYS_ABBR[dt.weekday()]
    mo = MONTHS_ABBR[dt.month - 1]
    tz_str = time.tzname[0] if time.daylight and time.localtime().tm_isdst else time.tzname[0]
    print(f"{wd} {mo} {dt.day:2d} {dt.hour:02d}:{dt.minute:02d}:{dt.second:02d} {tz_str} {dt.year}")
    return 0


def parse_date_argument(value: str, reference: datetime.datetime) -> datetime.datetime:
    text = value.strip()
    lowered = text.lower()
    if lowered in ("now", "today"):
        return reference
    if lowered in ("yesterday", "tomorrow"):
        offset = -1 if lowered == "yesterday" else 1
        return reference + datetime.timedelta(days=offset)

    try:
        parsed = datetime.datetime.fromisoformat(text.replace("Z", "+00:00"))
        if parsed.tzinfo is None:
            return parsed.replace(tzinfo=datetime.timezone.utc)
        return parsed.astimezone(datetime.timezone.utc)
    except ValueError:
        pass

    datetime_formats = ("%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M:%S")
    for date_format in datetime_formats:
        try:
            return datetime.datetime.strptime(text, date_format).replace(tzinfo=datetime.timezone.utc)
        except ValueError:
            continue

    date_formats = ("%Y/%m/%d", "%d %b %Y", "%b %d, %Y", "%B %d, %Y", "%d %B %Y")
    for date_format in date_formats:
        try:
            return datetime.datetime.strptime(text, date_format).replace(tzinfo=datetime.timezone.utc)
        except ValueError:
            continue

    match = re.fullmatch(r"([+-])\s*(\d+)\s+([A-Za-z]+)", text)
    if match:
        sign = -1 if match.group(1) == "-" else 1
        amount = int(match.group(2)) * sign
        unit = match.group(3).lower()
        multipliers = {
            "second": ("seconds", 1), "seconds": ("seconds", 1), "sec": ("seconds", 1), "s": ("seconds", 1),
            "minute": ("seconds", 60), "minutes": ("seconds", 60), "min": ("seconds", 60), "m": ("seconds", 60),
            "hour": ("seconds", 3600), "hours": ("seconds", 3600), "hr": ("seconds", 3600), "h": ("seconds", 3600),
            "day": ("days", 1), "days": ("days", 1), "d": ("days", 1),
            "week": ("days", 7), "weeks": ("days", 7), "w": ("days", 7),
            "month": ("days", 30), "months": ("days", 30), "mon": ("days", 30),
            "year": ("days", 365), "years": ("days", 365), "yr": ("days", 365), "y": ("days", 365),
        }
        if unit in multipliers:
            quantity, multiplier = multipliers[unit]
            delta = datetime.timedelta(**{quantity: amount * multiplier})
            return reference + delta

    raise ValueError("unrecognized date")


def format_custom(dt: datetime.datetime, fmt: str) -> str:
    """Custom date format string with % specifiers."""
    result = []
    i = 0
    while i < len(fmt):
        if fmt[i] == '%' and i + 1 < len(fmt):
            spec = fmt[i + 1]
            if spec == 'Y':
                result.append(f"{dt.year:04d}")
            elif spec == 'y':
                result.append(f"{dt.year % 100:02d}")
            elif spec == 'm':
                result.append(f"{dt.month:02d}")
            elif spec == 'd':
                result.append(f"{dt.day:02d}")
            elif spec == 'H':
                result.append(f"{dt.hour:02d}")
            elif spec == 'I':
                h = dt.hour % 12
                if h == 0:
                    h = 12
                result.append(f"{h:02d}")
            elif spec == 'M':
                result.append(f"{dt.minute:02d}")
            elif spec == 'S':
                result.append(f"{dt.second:02d}")
            elif spec == 's':
                result.append(str(int(dt.timestamp())))
            elif spec == 'A':
                result.append(WEEKDAYS[dt.weekday()])
            elif spec == 'a':
                result.append(WEEKDAYS_ABBR[dt.weekday()])
            elif spec == 'B':
                result.append(MONTHS[dt.month - 1])
            elif spec == 'b':
                result.append(MONTHS_ABBR[dt.month - 1])
            elif spec == 'Z':
                tz = time.tzname[0]
                result.append(tz)
            elif spec == 'z':
                off = dt.utcoffset()
                if off is not None:
                    total_min = int(off.total_seconds() // 60)
                    sign = '+' if total_min >= 0 else '-'
                    hours = abs(total_min) // 60
                    mins = abs(total_min) % 60
                    result.append(f"{sign}{hours:02d}{mins:02d}")
                else:
                    result.append("+0000")
            elif spec == 'p':
                result.append("AM" if dt.hour < 12 else "PM")
            elif spec == 'j':
                result.append(f"{dt.timetuple().tm_yday:03d}")
            elif spec == 'w':
                result.append(str(dt.weekday()))
            elif spec == 'u':
                result.append(str((dt.weekday() + 1) % 7 or 7))
            elif spec == 'V':
                iso_week = dt.isocalendar()[1]
                result.append(f"{iso_week:02d}")
            elif spec == 'C':
                result.append(f"{dt.year // 100:02d}")
            elif spec == 'D':
                result.append(dt.strftime("%m/%d/%y"))
            elif spec == 'F':
                result.append(dt.strftime("%Y-%m-%d"))
            elif spec == 'T':
                result.append(dt.strftime("%H:%M:%S"))
            elif spec == 'r':
                result.append(dt.strftime("%I:%M:%S %p"))
            elif spec == 'R':
                result.append(dt.strftime("%H:%M"))
            elif spec == '%':
                result.append('%')
            elif spec == 'n':
                result.append('\n')
            elif spec == 't':
                result.append('\t')
            else:
                result.append(f"%{spec}")
            i += 2
        else:
            result.append(fmt[i])
            i += 1
    return "".join(result)
