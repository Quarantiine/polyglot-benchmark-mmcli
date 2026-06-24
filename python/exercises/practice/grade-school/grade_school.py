class School:
    def __init__(self):
        self._roster = {}
        self._added_results = []

    def add_student(self, name, grade):
        # Check if student is already in any grade
        for g in self._roster:
            if name in self._roster[g]:
                self._added_results.append(False)
                return False
        
        if grade not in self._roster:
            self._roster[grade] = []
        
        self._roster[grade].append(name)
        self._added_results.append(True)
        return True

    def roster(self):
        sorted_grades = sorted(self._roster.keys())
        full_roster = []
        for grade in sorted_grades:
            for student in sorted(self._roster[grade]):
                full_roster.append(student)
        return full_roster

    def grade(self, grade_number):
        return sorted(self._roster.get(grade_number, []))

    def added(self):
        return self._added_results
