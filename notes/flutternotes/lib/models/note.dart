class Note {
  final int id;
  final String title;
  final String content;
  final DateTime createdAt;

  const Note({
    required this.id,
    required this.title,
    required this.content,
    required this.createdAt,
  });

  Note withId(int newId) => Note(
        id: newId,
        title: title,
        content: content,
        createdAt: createdAt,
      );

  Map<String, dynamic> toJson() => {
        'id': id,
        'title': title,
        'content': content,
        'createdAt': createdAt.toIso8601String(),
      };

  factory Note.fromJson(Map<String, dynamic> json) => Note(
        id: json['id'] as int,
        title: json['title'] as String,
        content: json['content'] as String,
        createdAt: DateTime.parse(json['createdAt'] as String),
      );

  @override
  bool operator ==(Object other) =>
      other is Note &&
      id == other.id &&
      title == other.title &&
      content == other.content &&
      createdAt == other.createdAt;

  @override
  int get hashCode => Object.hash(id, title, content, createdAt);

  @override
  String toString() =>
      'Note{id: $id, title: $title, content: $content, createdAt: $createdAt}';
}
