import { Controller, Get, Param, ParseIntPipe, Post } from '@nestjs/common';
import { BaseController } from '../library/Base.controller';
import { ExamService } from './exam.service';

@Controller("exam")
export class ExamController extends BaseController {
  constructor(private readonly examService: ExamService) {
    super();
  }

  @Post(":idExam/start")
  public async startExam(@Param("idExam", ParseIntPipe) idExam: number): Promise<any> {
    const user = super.getUserFromToken();

    void this.examService.emitStartExam({
      id_student: user.idStudent,
      id_exam: idExam,
    });

    return true;
  }

  @Post(":idExam/question/:idQuestion/answer/:idAnswer")
  public async answerQuestion(
    @Param("idExam", ParseIntPipe) idExam: number,
    @Param("idQuestion", ParseIntPipe) idQuestion: number,
    @Param("idAnswer", ParseIntPipe) idAnswer: number
  ): Promise<any> {
    const user = super.getUserFromToken();

    void this.examService.sendQuestionAnswer({
      id_student_exam: idExam,
      id_question: idQuestion,
      id_answer: idAnswer,
      id_student: user.idStudent,
    });

    return true;
  }

  @Post(":idExam/finish")
  public async finishExam(@Param("idExam", ParseIntPipe) idExam: number): Promise<any> {
    const user = super.getUserFromToken();

    void this.examService.finishExam({
      id_student_exam: idExam,
      id_student: user.idStudent,
    });

    return true;
  }
}
