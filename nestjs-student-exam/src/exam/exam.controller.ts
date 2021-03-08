import { Controller, Get, InternalServerErrorException, Param, ParseIntPipe, Post } from '@nestjs/common';
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

    try {
      const result = await this.examService.emitStartExam({
        id_student: user.idStudent,
        id_exam: idExam,
      })

      return { message: result };
    } catch (error) {
      throw new InternalServerErrorException(error);
    }
  }

  @Post(":idExam/question/:idQuestion/answer/:idAnswer")
  public async answerQuestion(
    @Param("idExam", ParseIntPipe) idExam: number,
    @Param("idQuestion", ParseIntPipe) idQuestion: number,
    @Param("idAnswer", ParseIntPipe) idAnswer: number
  ): Promise<any> {
    const user = super.getUserFromToken();
    try {
      const result = await this.examService.sendQuestionAnswer({
        id_student_exam: idExam,
        id_question: idQuestion,
        id_answer: idAnswer,
        id_student: user.idStudent,
      });

      return { message: result };
    } catch (error) {
      throw new InternalServerErrorException(error);
    }
  }

  @Post(":idExam/finish")
  public async finishExam(@Param("idExam", ParseIntPipe) idExam: number): Promise<any> {
    const user = super.getUserFromToken();

    try {
      const result = await this.examService.finishExam({
        id_student_exam: idExam,
        id_student: user.idStudent,
      });

      return { message: result };
    } catch (error) {
      throw new InternalServerErrorException(error);
    }
  }
}
